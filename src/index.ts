import colors from "ansi-colors";
import { Queue } from "async-await-queue";
import cliProgress from "cli-progress";
import filetype from "magic-bytes.js";

interface GifRecord {
    name: string;
    url: string;
    width: number;
    height: number;
}

const pb = new cliProgress.SingleBar({
    format:
        "CLI Progress |" +
        colors.cyan("{bar}") +
        "| {percentage}% || {value}/{total} Chunks",
    barCompleteChar: "\u2588",
    barIncompleteChar: "\u2591",
    hideCursor: true,
});

const file = await Bun.file("src/binary.bin.json").text();
const records: GifRecord[] = (JSON.parse(file) as any[]).map((r) => ({
    name: r["1"].split("/").slice(-1)[0],
    url: r["2"]["2"],
    width: r["2"]["3"],
    height: r["2"]["4"],
}));

const noLongerFound: string[] = [];

async function saveFile(record: GifRecord) {
    const buffer = await fetch(record.url).then((r) => r.arrayBuffer());
    const fileType = filetype(new Uint8Array(buffer));
    console.log(fileType);
    console.log(record.name);

    if (fileType.length === 0) {
        console.error(`Image ${record.name} no longer exists :(`);
        noLongerFound.push(record.name);
        return;
    }

    // Write file
    const hasher = new Bun.CryptoHasher("sha256");
    hasher.update(buffer);
    const hash = hasher.digest("hex").slice(0, 8);

    const extension = fileType[0].typename;
    const name = record.name.replace(`.${extension}`, "");
    const finalName = `${name}-${hash}.${extension}`;

    await Bun.write(`output/${finalName}`, buffer);
}

const myq = new Queue(5, 100);

pb.start(records.length, 0);
for (const record of records) {
    const me = Symbol();
    myq.wait(me);

    await saveFile(record);
    pb.increment();
}

pb.stop();
console.log("No longer found", noLongerFound);

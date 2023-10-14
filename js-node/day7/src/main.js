import { open } from "node:fs/promises";
import { argv } from "node:process";

class FileSystem {
  /** @type {FileSystemEntry} */
  root;
  /** @type {FileSystemEntry} */
  currentWorkingDirectory;

  constructor() {
    this.root = FileSystemEntry.newDirectory("/");
  }

  changeDirectory() {}

  list() {}
}

class FileSystemEntry {
  /** @type {string} */
  name;
  /** @type {boolean} */
  isDirectory;
  /** @type {number} */
  #size = 0;
  /** @type {FileSystemEntry[]} */
  directoryEntries = [];
  /** @type {FileSystemEntry | undefined} */
  parent;

  get size() {
    if (this.isDirectory) {
      return this.directoryEntries.reduce((sum, entry) => sum + entry.size, 0);
    } else {
      return this.#size;
    }
  }

  /**
   * @param {string} name
   * @param {number} size
   */
  static newFile(name, size) {
    const entry = new FileSystemEntry();

    entry.name = name;
    entry.#size = size;
    entry.isDirectory = false;

    return entry;
  }

  /**
   * @param {string} name
   */
  static newDirectory(name) {
    const entry = new FileSystemEntry();

    entry.name = name;
    entry.isDirectory = true;

    return entry;
  }

  /**
   * @param {FileSystemEntry} entry
   */
  addEntry(entry) {
    if (this.isDirectory) {
      this.directoryEntries.push(entry);
    } else {
      throw new Error("Cannot add filesystem entry to a file.");
    }
  }
}

const filePath = argv.at(2);
const input = await open(filePath);
const filesystem = new FileSystem();

for await (const line of input.readLines()) {
  if (line.startsWith("$")) {
    // Handle commands
    if (line === "$ cd /") {
      filesystem.currentWorkingDirectory = filesystem.root;
    } else if (line === "$ cd ..") {
      filesystem.currentWorkingDirectory =
        filesystem.currentWorkingDirectory.parent;
    } else if (line.startsWith("$ cd")) {
      const directoryName = line.substring(5);
      const directory =
        filesystem.currentWorkingDirectory.directoryEntries.find(
          (entry) => entry.name === directoryName,
        );

      if (!directory) {
        throw new Error(`Cannot change to directory ${directoryName}`);
      }

      filesystem.currentWorkingDirectory = directory;
    }
  } else {
    // Create file and add entry
    if (line.startsWith("dir")) {
      const directoryName = line.substring(4);
      const entry = FileSystemEntry.newDirectory(directoryName);

      entry.parent = filesystem.currentWorkingDirectory;
      filesystem.currentWorkingDirectory.addEntry(entry);
    } else {
      const [fileSize, fileName] = line.split(" ");
      const entry = FileSystemEntry.newFile(fileName, Number(fileSize));

      entry.parent = filesystem.currentWorkingDirectory;
      filesystem.currentWorkingDirectory.addEntry(entry);
    }
  }
}

await input.close();

// Traverse filesystem and add directories with size of at most 100_000 to sum
let sumOfTotalSizes = 0;
const directoriesToCheck = [filesystem.root];

while (directoriesToCheck.length > 0) {
  const currentDir = directoriesToCheck.pop();

  if (currentDir.size <= 100_000) {
    sumOfTotalSizes += currentDir.size;
  }

  for (const child of currentDir.directoryEntries) {
    if (child.isDirectory) {
      directoriesToCheck.push(child);
    }
  }
}

console.log(
  `Sum of total sizes of directories that should be deleted: ${sumOfTotalSizes}`,
);

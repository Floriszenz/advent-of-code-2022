import { open } from "node:fs/promises";

export class FileSystem {
  /** @type {FileSystemEntry} */
  root;
  /** @type {FileSystemEntry} */
  currentWorkingDirectory;

  constructor() {
    this.root = FileSystemEntry.newDirectory("/");
  }

  /**
   * @param {string} terminalOutputFile
   */
  static async build(terminalOutputFile) {
    const input = await open(terminalOutputFile);
    const filesystem = new FileSystem();

    for await (const line of input.readLines()) {
      if (line.startsWith("$")) {
        // Handle commands
        if (line.startsWith("$ cd")) {
          filesystem.changeDirectory(line.substring(5));
        }
      } else {
        // Create file and add entry
        if (line.startsWith("dir")) {
          const directoryName = line.substring(4);

          filesystem.makeDirectory(directoryName);
        } else {
          const [fileSize, fileName] = line.split(" ");

          filesystem.touch(fileName, Number(fileSize));
        }
      }
    }

    await input.close();

    return filesystem;
  }

  /**
   * @param {"/" | ".." | string} directory
   */
  changeDirectory(directory) {
    if (directory === "/") {
      this.currentWorkingDirectory = this.root;
    } else if (directory === "..") {
      this.currentWorkingDirectory = this.currentWorkingDirectory.parent;
    } else {
      const childDirectory = this.currentWorkingDirectory.directoryEntries.find(
        (entry) => entry.name === directory,
      );

      if (!childDirectory) {
        throw new Error(`Cannot change to directory ${directory}`);
      }

      this.currentWorkingDirectory = childDirectory;
    }
  }

  /**
   * @param {string} name
   */
  makeDirectory(name) {
    const entry = FileSystemEntry.newDirectory(name);

    entry.parent = this.currentWorkingDirectory;
    this.currentWorkingDirectory.addEntry(entry);
  }

  /**
   * @param {string} name
   * @param {number} size
   */
  touch(name, size) {
    const entry = FileSystemEntry.newFile(name, size);

    entry.parent = this.currentWorkingDirectory;
    this.currentWorkingDirectory.addEntry(entry);
  }

  /**
   * @template T
   * @param {(aggregatedValue: T, currentDirectory: FileSystemEntry) => T} aggregateFn
   * @param {T} initialValue
   */
  traverseFilesystem(aggregateFn, initialValue) {
    const directoriesToCheck = [this.root];
    let aggregatedValue = initialValue;

    while (directoriesToCheck.length > 0) {
      const currentDir = directoriesToCheck.pop();

      aggregatedValue = aggregateFn(aggregatedValue, currentDir);

      for (const child of currentDir.directoryEntries) {
        if (child.isDirectory) {
          directoriesToCheck.push(child);
        }
      }
    }

    return aggregatedValue;
  }
}

export class FileSystemEntry {
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
      throw new Error("Cannot add this entry to a file.");
    }
  }
}

import { argv } from "node:process";

import { FileSystem } from "./filesystem.js";

const filePath = argv.at(2);
const filesystem = await FileSystem.build(filePath);

const totalSizeOfSmallDirectories = filesystem.traverseFilesystem(
  (sum, currentDirectory) => {
    if (currentDirectory.size <= 100_000) {
      return sum + currentDirectory.size;
    }

    return sum;
  },
  0,
);

console.log(
  `Total size of directories that are each at most 100'000 bytes big: ${totalSizeOfSmallDirectories}`,
);

const AVAILABLE_DISK_SPACE = 70_000_000;
const REQUIRED_DISK_SPACE = 30_000_000;
const unusedDiskSpace = AVAILABLE_DISK_SPACE - filesystem.root.size;
const diskSpaceThatMustBeFreed = REQUIRED_DISK_SPACE - unusedDiskSpace;

const totalSizeOfDirectoryToDelete = filesystem.traverseFilesystem(
  (minimum, currentDirectory) => {
    if (
      currentDirectory.size >= diskSpaceThatMustBeFreed &&
      currentDirectory.size < minimum
    ) {
      return currentDirectory.size;
    }

    return minimum;
  },
  Infinity,
);

console.log(
  `Total size of directory that should be deleted: ${totalSizeOfDirectoryToDelete}`,
);

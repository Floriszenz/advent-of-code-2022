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

import * as fs from 'fs';
import * as rd from 'readline'

interface file {
    name: String,
    size: number
}

interface directory {
    name: String,
    files: file[],
    subDirectories: directory[],
    parentDirectory: directory | undefined,
    totalSize: number
}

const part1 = (directorySizes: number[]) => {
    let sum = 0;
    directorySizes.forEach(size => {
        if(size <= 100000) {
            sum += size;
        }
    });
    console.log("part1: " + sum);
}

const part2 = (directorySizes: number[]) => {
    directorySizes = directorySizes.sort((a, b) => a - b);
    const used = directorySizes[directorySizes.length - 1];
    const max = 70000000;
    const min = 30000000;

    const target = min - (max - used);

    const valids = directorySizes.filter(x => x >= target);
    const smallest = valids.sort((a, b) => a - b)[0];
    console.log("part2: " + smallest);
}

const readFile = () => {
    const lines: String[] = [];
    const reader = rd.createInterface(fs.createReadStream("input.txt"));

    reader.on("line", (l: string) => {
        lines.push(l);
    });

    reader.on("close", () => {
        const dir = parseFile(lines);
        part1(getDirectorySizes(dir));
        part2(getDirectorySizes(dir));
    });
}

const parseFile = (lines: String[]) => {
    let dir: directory = {
        name: "/",
        files: [],
        subDirectories: [],
        parentDirectory: undefined,
        totalSize: 0
    };

    lines.forEach(line => {
        const res = line.split(" ");
        if(res[0] === "$" && res[1] === "cd") {
            if(res[2] === ".." && dir.parentDirectory) {
                dir = dir.parentDirectory;
            }
            else if(res[2] === "/") {
                while(dir.parentDirectory) {
                    dir = dir.parentDirectory;
                }
            }
            else {
                const index = dir.subDirectories.findIndex(x => x.name === res[2]);
                dir = dir.subDirectories[index];
            }
        }
        else if(res[0] === "dir") {
            if(!dir.subDirectories.find(x => x.name === res[1])) {
                dir.subDirectories.push({
                    name: res[1],
                    files: [],
                    subDirectories: [],
                    parentDirectory: dir,
                    totalSize: 0
                });
            }
        }
        else if(res[0] != "$") {
            const name = res[1];
            const size = parseInt(res[0]);
            dir.files.push({name, size});
        }
    });
    
    while(dir.parentDirectory) {
        dir = dir.parentDirectory;
    }
    calculateSize(dir);
    return dir;
}

const calculateSize = (dir: directory) => {
    dir.files.forEach(x => dir.totalSize += x.size);
    dir.subDirectories.forEach(sub => dir.totalSize += calculateSize(sub));
    return dir.totalSize;
}

const printTree = (dir: directory) => {
    const printTreeRec = (dir: directory, tab: String) => {
        console.log(tab + "- name: " + dir.name);
        console.log(tab + "- size: " + dir.totalSize);
        console.log(tab + "- files:");
        dir.files.forEach(file => {
            console.log(tab + "    " + file.name)
            console.log(tab + "    " + file.size);
        });
        console.log(tab + "- directories:");
        dir.subDirectories.forEach(sub => {
            printTreeRec(sub, tab + "    ");
        });
    }
    printTreeRec(dir, "");
}

const getDirectorySizes = (dir: directory) => {
    const getDirectorySizesRec = (dir: directory, arr: number[]) => {
        arr.push(dir.totalSize);
        dir.subDirectories.forEach(sub => {
            getDirectorySizesRec(sub, arr);
        });
    }

    const arr: number[] = [];
    getDirectorySizesRec(dir, arr);
    return arr;
}

readFile();
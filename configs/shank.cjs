const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "mpl_engraver_program",
  programId: "ENGRVY4DL6uKDnNS91hCkJMwzTfcofYpkZH8zsgJfzA3",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "engraver"),
});

// @ts-ignore
import { wasm, WasmTester } from "circom_tester";
import { buildPoseidon } from "circomlibjs";
// @ts-ignore
import { ZqField } from "ffjavascript";
import { describe, beforeAll, test } from "bun:test";

describe("init", () => {
  const circ_file = "src/init.circom";
  let circ: WasmTester;

  beforeAll(async () => {
    circ = await wasm(circ_file);
    await circ.loadConstraints();
  });

  test("correctly computes hash", async () => {
    const alignment_array = [1, 2, 3, 4, 5, 6, 7, 8];

    const holon_hash = await calculateHash(alignment_array);

    const input = { alignment_array, holon_hash };

    const witness = await circ.calculateWitness(input);
    await circ.checkConstraints(witness);
  });

  test("should fail for wrong hash", async () => {
    const alignment_array = [1, 2, 3, 4, 5, 6, 7, 8];

    const holon_hash = 123n;

    const input = { alignment_array, holon_hash };
    try {
      const witness = await circ.calculateWitness(input);
    } catch (e) {
      return 0;
    }

    throw new Error("should have failed");
  });

  test("should fail if alignment array is not u8", async () => {
    try {
      for (let i = 0; i < 8; i++) {
        let alignment_array = [1, 2, 3, 4, 5, 6, 7, 8];
        alignment_array[i] = 300;

        const holon_hash = await calculateHash(alignment_array);

        const input = { alignment_array, holon_hash };
        const witness = await circ.calculateWitness(input);
      }
    } catch (e) {
      return 0;
    }

    throw new Error("should have failed");
  });
});

async function calculateHash(alignment_array: number[]) {
  const SNARK_FIELD_SIZE =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";
  const F = new ZqField(SNARK_FIELD_SIZE);

  const poseidonEx = await buildPoseidon();
  const holon_hash = poseidonEx.F.toObject(
    poseidonEx(alignment_array, F.zero, 1)
  );

  return holon_hash;
}

pragma circom 2.0.0;

include "../../node_modules/circomlib/circuits/poseidon.circom";
include "../../node_modules/circomlib/circuits/bitify.circom";

template Init() {
    signal input alignment_array[8];
    signal input holon_hash;

    // check that values of alignment array are u8
    component u8_check[8];
    for (var i = 0; i < 8; i++) {
        u8_check[i] = Num2Bits(8);
        u8_check[i].in <== alignment_array[i];
    }

    // compute poseidon hash
    component poseidon_hash = Poseidon(8);
    for (var i = 0; i < 8; i++) {
        poseidon_hash.inputs[i] <== alignment_array[i];
    }

    poseidon_hash.out === holon_hash;

}

component main {public [holon_hash]} = Init();
pragma circom 2.1.0;

include "../../node_modules/circomlib/circuits/poseidon.circom";
include "../../node_modules/circomlib/circuits/bitify.circom";

template Init() {
    signal input alignment_array[8];
    signal input holon_hash;

    // check that values of alignment array are u8
    for (var i = 0; i < 8; i++) {
        _ <== Num2Bits(8)(alignment_array[i]);
    }

    // compute and check poseidon hash
    signal poseidon_hash <== Poseidon(8)(alignment_array);
    poseidon_hash === holon_hash;

}

component main {public [holon_hash]} = Init();

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/*
    Muito semelhante a struct do C, o que é ótimo pela familiaridade, mas 
    sem tratamento de ponteiros.
*/
struct data_test {
    uint256 id;
    string  name;
    address user;
}

contract SolidityTypes {
    // State Variable
    string public hello = "HelloW!";
    string public world = "World!" ;

    // It has a some differences from string data type. This type has a preset size of bytes
    // as well as being raw bytes.
    bytes32 public hello32 = "123456789,123456789,123456789,12";
    bytes31 public hello31 = "123456789,123456789,123456789,1";
    bytes30 public hello30 = "123456789,123456789,123456789,";
    bytes29 public hello29 = "123456789,123456789,123456789";
    bytes29 public hello_bytes = "Hello, Bytes!";

    // Type used to store user address
    address public user = 0x4B20993Bc481177ec7E8f571ceCaE8A9e22C02db;

    // Instance
    data_test public my_data = data_test(
        1,
        "First, Struct",
        0x4B20993Bc481177ec7E8f571ceCaE8A9e22C02db
    );
}
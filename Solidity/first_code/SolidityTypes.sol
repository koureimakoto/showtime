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

struct Vec2 {
    int256 x_axis;
    int256 y_axis;
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

    // Array
    uint[]    public my_number_array = [1, 2, 3];
    string[2] public string_array = ["Makoto", "World"];
    string[2] public  sized_mutable_string;
    uint      private size_of_mutable_string = 0;

    string[]  public mutable_string;


    // Array with size presetted were cheaper than dynamic array
    function sized_push(string memory _value) public {
        if(size_of_mutable_string >= 2) {
            sized_mutable_string[size_of_mutable_string] = _value;
            size_of_mutable_string += 1;
        }
    }

    // I will avoid, but even so it's sometimes is necessary. 
    function unsized_push(string memory _value) public {
        if(mutable_string.length >= 2) {
            mutable_string.push(_value);
        }
    }


    // Coord
    uint256 public hypotenuse = 0;

    function distance(Vec2 memory lhs, Vec2 memory rhs) public {
        uint256 cathetus_x = uint256(rhs.x_axis - lhs.x_axis);
        uint256 cathetus_y = uint256(rhs.y_axis - lhs.y_axis);

        if(cathetus_x < cathetus_y) {
            cathetus_x = cathetus_x ^ cathetus_y;
            cathetus_y = cathetus_y ^ cathetus_x;
            cathetus_x = cathetus_x ^ cathetus_y;
        }

        hypotenuse = cathetus_x + (cathetus_y * cathetus_y / cathetus_x) / 2 ;
        // hypotenuse = sqrt(cathetus_x**2 + cathetus_y**2);
    }

    // https://github.com/Uniswap/v2-core/blob/v1.0.1/contracts/libraries/Math.sol
    function sqrt(uint256 value) internal pure returns (uint256) {
        if(value < 2) {
            return value;
        }
        
        uint256 result = value;
        uint256 buff = (value / 2) + 1;

        while(buff < result) {
            result = buff;
            buff   = ((value / buff) + buff) / 2;
        }

        return result;
    }


    // Another test using array
    // Para acesso ao test é semelhante ao passagem de marametro em tuplas no Remix
    // na orde de um array normal seguidos de outros arrays.
    uint256[2][2] public array_2d = [[1,2], [3,4]];

}
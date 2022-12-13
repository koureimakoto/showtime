// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.0;

// struct GLTF {
//     string version;
//     string description;
//     bytes[] maps;
//     bytes   glb;
// }

// struct Image {
//     string title;
//     string description;
//     bytes image;
// }


// struct NFT2D {
//     string name;
//     address owner;
//     // Image nft;
// }

// struct NFT3D {
//     string name;
//     address owner;
//     GLTF nft;
// }

struct Raw2DNFT{
    address owner;
    string  name;
    string  mime;
    uint256 time;
}

struct Collection {
    string  path;
    uint256 payment;
}

contract TallesNft {
    

    // Duas redes encadeadas
    // Primeira NFT : Segunda Owner


    mapping(
        address => Raw2DNFT[]
    ) public nfts;
    uint256 private nfts_counter;

    mapping(
        address => mapping(
            string => Collection[]
        )
    ) public owners;
    uint256 private owners_counter;

    function mint(string memory _name, string memory _mime) public {
        nfts[msg.sender].push(Raw2DNFT (
            msg.sender,
            _name,
            _mime,
            block.timestamp
        ));
    }

    // function transfer(address _to, uint256 nft) public {
        
    //     address owner = msg.sender;
    //     string owner_nft = owner[owner][]


    // }

}
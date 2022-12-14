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
    address path;
    uint256 id_token;
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
        address => Collection[]
    ) public owners;
    uint256 private owners_counter;

    address      internal nft_buffer_owner;
    Collection   internal nft_buffer_node;
    Collection[] internal nft_collection;

    modifier mustBeCurrentOwner(uint256 _id_token) {
        address self_owner    = msg.sender;
        nft_buffer_node = owners[self_owner][_id_token];
        nft_buffer_owner= nfts[nft_buffer_node.path][nft_buffer_node.id_token].owner;

        require(nft_buffer_owner == self_owner);
        _;
    }

    function mint(string memory _name, string memory _mime) public {
        nfts[msg.sender].push(Raw2DNFT (
            msg.sender,
            _name,
            _mime,
            block.timestamp
        ));
    }

    function transfer(address _to, uint256 _id_token) public
        mustBeCurrentOwner(_id_token) {
        
        require(msg.sender != _to);

        
        for(uint i = _id_token; i < owners[msg.sender].length - 1; i++) {
            nft_collection[i] = owners[msg.sender][i + 1];
        }

        owners[msg.sender] = nft_collection;
        nfts[nft_buffer_node.path][_id_token].owner = _to;

    }

}
// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.0;

/**
 * 
 * Test interessante para concatenção. Era apenas para usar mapping, ai extendi o mapping
 * Depois queria concatenar. Agora quero testar retorno em indice se possível.
 * 
 */
contract ContractWithMapping {
    // mappings
    mapping(uint => string) public users;
    uint users_count = 0;

    mapping(string =>  string[]) public user_children;

    function add_user(string memory name) public {
        users[users_count++] = name;
    }

    function add_user_using_only_string(string memory father_name, string memory child_name) public {
        user_children[father_name].push(child_name);
    }
    
    function get_family() public view returns(string memory) {
        string memory buffer = "";

        for(uint8 i = 0; i < users_count; i++) {
            string memory father = string(
                abi.encodePacked(
                    "[", users[i], "]: "
                )
            );

            string memory children_format = "";
            for(uint8 j = 0; j < user_children[users[i]].length; j++) {
                children_format = string(
                    abi.encodePacked(
                         children_format, user_children[users[i]][j], ", "
                    )
                );
            }

            buffer = string(
                abi.encodePacked(
                    buffer, father, children_format , ". |"
                )
            );

        }
        
        return buffer;
    }

}
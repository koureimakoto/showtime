// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.0;

contract Counter {
    uint256
        count;
        
    uint  public
        super_count = 0;

    
    constructor(uint256 first) {
        count = first;
    }

    function get_count() public view returns (uint256) {
        return count;
    }

    // Quando não tem view na função ele tem gas fee pela 
    // possibilidade de mudança de estado.
    // Olhando o debugger, parece que cada rotina interna que
    // determina uma mudança de estado tem uma pequeno valor de
    // gas atribuindo, sendo possivelmente o que venha a deter-
    // minar o gas fee total da transação. # Pesquisar mais ...
    function increment_count() public {
        count++;
        super_count += count * 2;
    }
}

/**
 * Ok, o negocio parece ser um pouco mais complicado do que previa
 * o gas fee é terminado por um valor de gas, como rotina, a rede 
 * os dados.
 * 
 * Parentemente, quanto mais enxuto for, mais economico.
 */
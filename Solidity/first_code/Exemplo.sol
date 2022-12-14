// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// https://www.youtube.com/watch?v=EhPeHeoKF88&t=2626s
contract Exemplo {
    /*
        Consegui entender bem o event.
    */
    event Occupy(address _occupant, uint256 _value);

    enum State {
        Vacant,
        Occupied
    }

    uint256 private constant PRICE = 2 ether;
    address payable public   owner;
    State   public           CurrentState;

    constructor() {
        owner = payable(msg.sender);
        CurrentState = State.Vacant;
    }


    modifier checkState() {
        require(CurrentState == State.Vacant, "Currently occupied");
        _;
    }

    modifier checkCosts() {
        require(msg.value >= PRICE, "Your poor!");
        _;
    }

    function book() public payable
        checkCosts
        checkState {
        CurrentState = State.Occupied;
        owner.transfer(msg.value);

        // Retorna apenas apra demonstrar os tipos de retorna
        // mas não estão sendo usados agora.
        (bool sent, bytes memory data) = owner.call {
            value: msg.value
        }("Msg");
        require(sent, "AI CARAMBA");

        emit Occupy(msg.sender, msg.value);
    }

}
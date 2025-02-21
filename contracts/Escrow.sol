// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract Escrow {
    address public seller;
    address public buyer;
    uint256 public amount;
    bool public isFunded;
    bool public isReleased;

    event Funded(address indexed buyer, uint256 amount);
    event Released(address indexed seller, uint256 amount);
    event Refunded(address indexed buyer, uint256 amount);

    modifier onlyBuyer() {
        require(msg.sender == buyer, "Only buyer can call this function");
        _;
    }

    modifier onlySeller() {
        require(msg.sender == seller, "Only seller can call this function");
        _;
    }

    constructor(address _buyer, address _seller, uint256 _amount) {
        buyer = _buyer;
        seller = _seller;
        amount = _amount;
    }

    function fund() external payable onlyBuyer {
        require(msg.value == amount, "Incorrect amount sent");
        require(!isFunded, "Escrow already funded");
        isFunded = true;
        emit Funded(msg.sender, msg.value);
    }

    function release() external onlySeller {
        require(isFunded, "Escrow not funded");
        require(!isReleased, "Funds already released");
        isReleased = true;
        payable(seller).transfer(amount);
        emit Released(seller, amount);
    }

    function refund() external onlyBuyer {
        require(isFunded, "Escrow not funded");
        require(!isReleased, "Funds already released");
        isFunded = false;
        payable(buyer).transfer(amount);
        emit Refunded(buyer, amount);
    }
}


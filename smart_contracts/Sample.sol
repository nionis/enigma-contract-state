pragma solidity ^0.5.0;
pragma experimental ABIEncoderV2;

contract Sample {
  bool public stateBool = false;

  function toggleBool() external {
    stateBool = true;
  }
}

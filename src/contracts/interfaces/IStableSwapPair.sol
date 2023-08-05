//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

pragma experimental ABIEncoderV2;

interface IStableSwapPair {
    function token0() external view returns (address);
    function token1() external view returns (address);

    function getReserves()
        external
        view
        returns (uint256 reserve0, uint256 reserve1, uint256 blockTimestampLast);

    function isStable() external view returns (bool);
}

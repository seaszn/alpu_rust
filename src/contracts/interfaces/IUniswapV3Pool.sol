//SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.19;

pragma experimental ABIEncoderV2;

struct UniswapV3State {
    uint160 sqrtPriceX96;
    int24 tick;
    uint128 liquidity;
}

interface IUniswapV3Pool {
    function token0() external view returns (address);

    function token1() external view returns (address);

    function liquidity() external view returns (uint128);

    function slot0()
        external
        view
        returns (
            uint160 sqrtPriceX96,
            int24 tick,
            uint16 observationIndex,
            uint16 observationCardinality,
            uint16 observationCardinalityNext,
            uint8 feeProtocol,
            bool unlocked
        );
}
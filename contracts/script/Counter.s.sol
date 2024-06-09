// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Counter} from "../src/Counter.sol";

contract CounterScript is Script {
    function setUp() public {}

    function run() public {
        // Acessa a variável de ambiente PRIVATE_KEY e converte para uint256
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Instancia e interage com o contrato Counter
        Counter counter = new Counter();
        console.log("Counter address: ", address(counter));
        counter.setNumber(69420);
        console.log("Counter number: ", counter.number());
        
        vm.stopBroadcast();
    }
}
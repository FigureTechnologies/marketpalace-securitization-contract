# Metadata Bilateral Exchange Kotlin Client

This is a [Kotlin](https://kotlinlang.org/)-based client that leverages [Provenance Blockchain's](https://provenance.io)
[PbClient](https://github.com/provenance-io/pb-grpc-client-kotlin) to make [GRPC](https://grpc.io/) requests to a
Provenance Blockchain instance of the [Metadata Bilateral Exchange smart contract](../smart-contract).

## Status
[![Latest Release][release-badge]][release-latest]
[![Maven Central][maven-badge]][maven-url]
[![Apache 2.0 License][license-badge]][license-url]

[maven-badge]: https://maven-badges.herokuapp.com/maven-central/io.provenance.bilateral/bilateral-client/badge.svg
[maven-url]: https://maven-badges.herokuapp.com/maven-central/io.provenance.bilateral/bilateral-client
[release-badge]: https://img.shields.io/github/tag/provenance-io/metadata-bilateral-exchange.svg
[release-latest]: https://github.com/provenance-io/metadata-bilateral-exchange/releases/latest
[license-badge]: https://img.shields.io/github/license/provenance-io/metadata-bilateral-exchange.svg
[license-url]: https://github.com/provenance-io/metadata-bilateral-exchange/blob/main/LICENSE

## Usage
The `BilateralContractClient` includes functions that allow communication with an instance of the Metadata Bilateral
Exchange smart contract.  It parallels all functionality, and it exposes data classes that translate to the request
JSON required by the contract.

Creating a new instance of the client:
```kotlin
import io.provenance.bilateral.client.BilateralContractClient
import io.provenance.bilateral.util.ContractAddressResolver
import io.provenance.client.grpc.GasEstimationMethod
import io.provenance.client.grpc.PbClient
import java.net.URI

fun makeClient(): BilateralContractClient = BilateralContractClient.builder(
    pbClient = PbClient(
        chainId = "chain-local",
        channelUri = URI.create("http://localhost:9090"),
        gasEstimationMethod = GasEstimationMethod.MSG_FEE_CALCULATION
    ),
    addressResolver = ContractAddressResolver.FromName("mycontractname.pb"),
).build()
```

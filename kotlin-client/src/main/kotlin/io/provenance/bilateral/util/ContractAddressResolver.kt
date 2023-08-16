package io.provenance.bilateral.util

import io.provenance.client.grpc.PbClient
import io.provenance.client.protobuf.extensions.resolveAddressForName

/**
 * Resolves a contract's address from the given input, leveraging the [PbClient] to retrieve an unknown address by
 * looking up the Provenance Blockchain name module value where necessary.
 */
sealed interface ContractAddressResolver {
    /**
     * A provided address.  This circumvents the name module lookup; the underlying code will assume the provided
     * address is correct.  This variant should always be used if the address is known.
     *
     * @param address A bech32 address that belongs to a Provenance Blockchain smart contract instance of the Metadata
     * Bilateral Exchange smart contract.
     */
    class ProvidedAddress(val address: String) : ContractAddressResolver

    /**
     * A reference to the Metadata Bilateral Exchange smart contract's bind name value.  This value is also found in
     * [io.provenance.bilateral.models.ContractInfo.bindName].
     *
     * @param name A Provenance Blockchain name module value to be used to resolve a bech32 address for the smart
     * contract.
     */
    class FromName(val name: String) : ContractAddressResolver

    /**
     * Attempts to fetch the Metadata Bilateral Exchange smart contract's bech32 address from using the given variant's
     * information.
     *
     * @param pbClient When a name is provided, this value is used to resolve it to the bech32 address of the smart
     * contract.  If an address was specified directly, this client instance is not used.
     */
    fun getAddress(pbClient: PbClient): String = when (this) {
        is ProvidedAddress -> this.address
        is FromName -> pbClient.nameClient.resolveAddressForName(this.name)
    }
}

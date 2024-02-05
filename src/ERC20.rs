pub use erc20::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod erc20 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"authorizer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"}],\"name\":\"AuthorizationCanceled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"authorizer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"}],\"name\":\"AuthorizationUsed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Blacklisted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address payable\",\"name\":\"relayerAddress\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"functionSignature\",\"type\":\"bytes\"}],\"name\":\"MetaTransactionExecuted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Pause\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newRescuer\",\"type\":\"address\"}],\"name\":\"RescuerChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\"}],\"name\":\"RoleAdminChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleGranted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleRevoked\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"UnBlacklisted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Unpause\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"APPROVE_WITH_AUTHORIZATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"BLACKLISTER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"CANCEL_AUTHORIZATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DEPOSITOR_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"EIP712_VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"META_TRANSACTION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"PAUSER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"PERMIT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"RESCUER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"TRANSFER_WITH_AUTHORIZATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"WITHDRAW_WITH_AUTHORIZATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"}],\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validAfter\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validBefore\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"approveWithAuthorization\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authorizer\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"}],\"name\":\"authorizationState\",\"outputs\":[{\"internalType\":\"enum GasAbstraction.AuthorizationState\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"blacklist\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"blacklisters\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authorizer\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"cancelAuthorization\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\"}],\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"decrement\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validAfter\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validBefore\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"decreaseAllowanceWithAuthorization\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"depositData\",\"type\":\"bytes\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"functionSignature\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"sigR\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"sigS\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"sigV\",\"type\":\"uint8\"}],\"name\":\"executeMetaTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"grantRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\"}],\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"increment\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validAfter\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validBefore\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"increaseAllowanceWithAuthorization\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newName\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"newSymbol\",\"type\":\"string\"},{\"internalType\":\"uint8\",\"name\":\"newDecimals\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"childChainManager\",\"type\":\"address\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"isBlacklisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pausers\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"permit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"renounceRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"tokenContract\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"rescueERC20\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"rescuers\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"revokeRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validAfter\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validBefore\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"transferWithAuthorization\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"unBlacklist\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"unpause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newName\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"newSymbol\",\"type\":\"string\"}],\"name\":\"updateMetadata\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validAfter\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"validBefore\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"nonce\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"withdrawWithAuthorization\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static ERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ERC20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ERC20)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `APPROVE_WITH_AUTHORIZATION_TYPEHASH` (0x10c8b404) function
        pub fn approve_with_authorization_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([16, 200, 180, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BLACKLISTER_ROLE` (0xf515e6f2) function
        pub fn blacklister_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([245, 21, 230, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CANCEL_AUTHORIZATION_TYPEHASH` (0xd9169487) function
        pub fn cancel_authorization_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([217, 22, 148, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH` (0x4b71f5c3) function
        pub fn decrease_allowance_with_authorization_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([75, 113, 245, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEPOSITOR_ROLE` (0xa3b0b5a3) function
        pub fn depositor_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([163, 176, 181, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EIP712_VERSION` (0xeccec5a8) function
        pub fn eip712_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([236, 206, 197, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH` (0xc7b84f8d) function
        pub fn increase_allowance_with_authorization_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([199, 184, 79, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `META_TRANSACTION_TYPEHASH` (0xfa2c322d) function
        pub fn meta_transaction_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([250, 44, 50, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PAUSER_ROLE` (0xe63ab1e9) function
        pub fn pauser_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 58, 177, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function
        pub fn permit_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESCUER_ROLE` (0x71372544) function
        pub fn rescuer_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([113, 55, 37, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TRANSFER_WITH_AUTHORIZATION_TYPEHASH` (0xa0cc6a68) function
        pub fn transfer_with_authorization_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([160, 204, 106, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WITHDRAW_WITH_AUTHORIZATION_TYPEHASH` (0x2004d291) function
        pub fn withdraw_with_authorization_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 4, 210, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveWithAuthorization` (0xe1560fd3) function
        pub fn approve_with_authorization(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            valid_after: ::ethers::core::types::U256,
            valid_before: ::ethers::core::types::U256,
            nonce: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [225, 86, 15, 211],
                    (owner, spender, value, valid_after, valid_before, nonce, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizationState` (0xe94a0102) function
        pub fn authorization_state(
            &self,
            authorizer: ::ethers::core::types::Address,
            nonce: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([233, 74, 1, 2], (authorizer, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blacklist` (0xf9f92be4) function
        pub fn blacklist(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 249, 43, 228], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blacklisters` (0x7cfa1cfa) function
        pub fn blacklisters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([124, 250, 28, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelAuthorization` (0x5a049a70) function
        pub fn cancel_authorization(
            &self,
            authorizer: ::ethers::core::types::Address,
            nonce: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 4, 154, 112], (authorizer, nonce, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowanceWithAuthorization` (0xb0086095) function
        pub fn decrease_allowance_with_authorization(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            decrement: ::ethers::core::types::U256,
            valid_after: ::ethers::core::types::U256,
            valid_before: ::ethers::core::types::U256,
            nonce: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [176, 8, 96, 149],
                    (
                        owner,
                        spender,
                        decrement,
                        valid_after,
                        valid_before,
                        nonce,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xcf2c52cb) function
        pub fn deposit(
            &self,
            user: ::ethers::core::types::Address,
            deposit_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 44, 82, 203], (user, deposit_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeMetaTransaction` (0x0c53c51c) function
        pub fn execute_meta_transaction(
            &self,
            user_address: ::ethers::core::types::Address,
            function_signature: ::ethers::core::types::Bytes,
            sig_r: [u8; 32],
            sig_s: [u8; 32],
            sig_v: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [12, 83, 197, 28],
                    (user_address, function_signature, sig_r, sig_s, sig_v),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowanceWithAuthorization` (0xc237108f) function
        pub fn increase_allowance_with_authorization(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            increment: ::ethers::core::types::U256,
            valid_after: ::ethers::core::types::U256,
            valid_before: ::ethers::core::types::U256,
            nonce: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 55, 16, 143],
                    (
                        owner,
                        spender,
                        increment,
                        valid_after,
                        valid_before,
                        nonce,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xde7ea79d) function
        pub fn initialize(
            &self,
            new_name: ::std::string::String,
            new_symbol: ::std::string::String,
            new_decimals: u8,
            child_chain_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [222, 126, 167, 157],
                    (new_name, new_symbol, new_decimals, child_chain_manager),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialized` (0x158ef93e) function
        pub fn initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBlacklisted` (0xfe575a87) function
        pub fn is_blacklisted(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([254, 87, 90, 135], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pausers` (0xe8d6fc47) function
        pub fn pausers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([232, 214, 252, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescueERC20` (0xb2118a8d) function
        pub fn rescue_erc20(
            &self,
            token_contract: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 17, 138, 141], (token_contract, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescuers` (0x100a1560) function
        pub fn rescuers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([16, 10, 21, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferWithAuthorization` (0xe3ee160e) function
        pub fn transfer_with_authorization(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            valid_after: ::ethers::core::types::U256,
            valid_before: ::ethers::core::types::U256,
            nonce: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 238, 22, 14],
                    (from, to, value, valid_after, valid_before, nonce, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unBlacklist` (0x1a895266) function
        pub fn un_blacklist(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 137, 82, 102], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMetadata` (0x56ce0e6c) function
        pub fn update_metadata(
            &self,
            new_name: ::std::string::String,
            new_symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 206, 14, 108], (new_name, new_symbol))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawWithAuthorization` (0x7b766e5f) function
        pub fn withdraw_with_authorization(
            &self,
            owner: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            valid_after: ::ethers::core::types::U256,
            valid_before: ::ethers::core::types::U256,
            nonce: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 118, 110, 95],
                    (owner, value, valid_after, valid_before, nonce, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AuthorizationCanceled` event
        pub fn authorization_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizationCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AuthorizationUsed` event
        pub fn authorization_used_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizationUsedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Blacklisted` event
        pub fn blacklisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlacklistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MetaTransactionExecuted` event
        pub fn meta_transaction_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MetaTransactionExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Pause` event
        pub fn pause_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauseFilter> {
            self.0.event()
        }
        ///Gets the contract's `RescuerChanged` event
        pub fn rescuer_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RescuerChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UnBlacklisted` event
        pub fn un_blacklisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnBlacklistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpause` event
        pub fn unpause_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpauseFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ERC20Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AuthorizationCanceled",
        abi = "AuthorizationCanceled(address,bytes32)"
    )]
    pub struct AuthorizationCanceledFilter {
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub nonce: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AuthorizationUsed", abi = "AuthorizationUsed(address,bytes32)")]
    pub struct AuthorizationUsedFilter {
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub nonce: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Blacklisted", abi = "Blacklisted(address)")]
    pub struct BlacklistedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "MetaTransactionExecuted",
        abi = "MetaTransactionExecuted(address,address,bytes)"
    )]
    pub struct MetaTransactionExecutedFilter {
        pub user_address: ::ethers::core::types::Address,
        pub relayer_address: ::ethers::core::types::Address,
        pub function_signature: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Pause", abi = "Pause()")]
    pub struct PauseFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RescuerChanged", abi = "RescuerChanged(address)")]
    pub struct RescuerChangedFilter {
        #[ethevent(indexed)]
        pub new_rescuer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "UnBlacklisted", abi = "UnBlacklisted(address)")]
    pub struct UnBlacklistedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Unpause", abi = "Unpause()")]
    pub struct UnpauseFilter;
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ERC20Events {
        ApprovalFilter(ApprovalFilter),
        AuthorizationCanceledFilter(AuthorizationCanceledFilter),
        AuthorizationUsedFilter(AuthorizationUsedFilter),
        BlacklistedFilter(BlacklistedFilter),
        MetaTransactionExecutedFilter(MetaTransactionExecutedFilter),
        PauseFilter(PauseFilter),
        RescuerChangedFilter(RescuerChangedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UnBlacklistedFilter(UnBlacklistedFilter),
        UnpauseFilter(UnpauseFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = AuthorizationCanceledFilter::decode_log(log) {
                return Ok(ERC20Events::AuthorizationCanceledFilter(decoded));
            }
            if let Ok(decoded) = AuthorizationUsedFilter::decode_log(log) {
                return Ok(ERC20Events::AuthorizationUsedFilter(decoded));
            }
            if let Ok(decoded) = BlacklistedFilter::decode_log(log) {
                return Ok(ERC20Events::BlacklistedFilter(decoded));
            }
            if let Ok(decoded) = MetaTransactionExecutedFilter::decode_log(log) {
                return Ok(ERC20Events::MetaTransactionExecutedFilter(decoded));
            }
            if let Ok(decoded) = PauseFilter::decode_log(log) {
                return Ok(ERC20Events::PauseFilter(decoded));
            }
            if let Ok(decoded) = RescuerChangedFilter::decode_log(log) {
                return Ok(ERC20Events::RescuerChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ERC20Events::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ERC20Events::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ERC20Events::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnBlacklistedFilter::decode_log(log) {
                return Ok(ERC20Events::UnBlacklistedFilter(decoded));
            }
            if let Ok(decoded) = UnpauseFilter::decode_log(log) {
                return Ok(ERC20Events::UnpauseFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizationCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AuthorizationUsedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlacklistedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MetaTransactionExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PauseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescuerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnBlacklistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpauseFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC20Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<AuthorizationCanceledFilter> for ERC20Events {
        fn from(value: AuthorizationCanceledFilter) -> Self {
            Self::AuthorizationCanceledFilter(value)
        }
    }
    impl ::core::convert::From<AuthorizationUsedFilter> for ERC20Events {
        fn from(value: AuthorizationUsedFilter) -> Self {
            Self::AuthorizationUsedFilter(value)
        }
    }
    impl ::core::convert::From<BlacklistedFilter> for ERC20Events {
        fn from(value: BlacklistedFilter) -> Self {
            Self::BlacklistedFilter(value)
        }
    }
    impl ::core::convert::From<MetaTransactionExecutedFilter> for ERC20Events {
        fn from(value: MetaTransactionExecutedFilter) -> Self {
            Self::MetaTransactionExecutedFilter(value)
        }
    }
    impl ::core::convert::From<PauseFilter> for ERC20Events {
        fn from(value: PauseFilter) -> Self {
            Self::PauseFilter(value)
        }
    }
    impl ::core::convert::From<RescuerChangedFilter> for ERC20Events {
        fn from(value: RescuerChangedFilter) -> Self {
            Self::RescuerChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ERC20Events {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ERC20Events {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ERC20Events {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UnBlacklistedFilter> for ERC20Events {
        fn from(value: UnBlacklistedFilter) -> Self {
            Self::UnBlacklistedFilter(value)
        }
    }
    impl ::core::convert::From<UnpauseFilter> for ERC20Events {
        fn from(value: UnpauseFilter) -> Self {
            Self::UnpauseFilter(value)
        }
    }
    ///Container type for all input parameters for the `APPROVE_WITH_AUTHORIZATION_TYPEHASH` function with signature `APPROVE_WITH_AUTHORIZATION_TYPEHASH()` and selector `0x10c8b404`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "APPROVE_WITH_AUTHORIZATION_TYPEHASH",
        abi = "APPROVE_WITH_AUTHORIZATION_TYPEHASH()"
    )]
    pub struct ApproveWithAuthorizationTypehashCall;
    ///Container type for all input parameters for the `BLACKLISTER_ROLE` function with signature `BLACKLISTER_ROLE()` and selector `0xf515e6f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BLACKLISTER_ROLE", abi = "BLACKLISTER_ROLE()")]
    pub struct BlacklisterRoleCall;
    ///Container type for all input parameters for the `CANCEL_AUTHORIZATION_TYPEHASH` function with signature `CANCEL_AUTHORIZATION_TYPEHASH()` and selector `0xd9169487`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "CANCEL_AUTHORIZATION_TYPEHASH",
        abi = "CANCEL_AUTHORIZATION_TYPEHASH()"
    )]
    pub struct CancelAuthorizationTypehashCall;
    ///Container type for all input parameters for the `DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH` function with signature `DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH()` and selector `0x4b71f5c3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH",
        abi = "DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH()"
    )]
    pub struct DecreaseAllowanceWithAuthorizationTypehashCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `DEPOSITOR_ROLE` function with signature `DEPOSITOR_ROLE()` and selector `0xa3b0b5a3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DEPOSITOR_ROLE", abi = "DEPOSITOR_ROLE()")]
    pub struct DepositorRoleCall;
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `EIP712_VERSION` function with signature `EIP712_VERSION()` and selector `0xeccec5a8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "EIP712_VERSION", abi = "EIP712_VERSION()")]
    pub struct Eip712VersionCall;
    ///Container type for all input parameters for the `INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH` function with signature `INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH()` and selector `0xc7b84f8d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH",
        abi = "INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH()"
    )]
    pub struct IncreaseAllowanceWithAuthorizationTypehashCall;
    ///Container type for all input parameters for the `META_TRANSACTION_TYPEHASH` function with signature `META_TRANSACTION_TYPEHASH()` and selector `0xfa2c322d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "META_TRANSACTION_TYPEHASH", abi = "META_TRANSACTION_TYPEHASH()")]
    pub struct MetaTransactionTypehashCall;
    ///Container type for all input parameters for the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "PAUSER_ROLE", abi = "PAUSER_ROLE()")]
    pub struct PauserRoleCall;
    ///Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    ///Container type for all input parameters for the `RESCUER_ROLE` function with signature `RESCUER_ROLE()` and selector `0x71372544`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "RESCUER_ROLE", abi = "RESCUER_ROLE()")]
    pub struct RescuerRoleCall;
    ///Container type for all input parameters for the `TRANSFER_WITH_AUTHORIZATION_TYPEHASH` function with signature `TRANSFER_WITH_AUTHORIZATION_TYPEHASH()` and selector `0xa0cc6a68`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "TRANSFER_WITH_AUTHORIZATION_TYPEHASH",
        abi = "TRANSFER_WITH_AUTHORIZATION_TYPEHASH()"
    )]
    pub struct TransferWithAuthorizationTypehashCall;
    ///Container type for all input parameters for the `WITHDRAW_WITH_AUTHORIZATION_TYPEHASH` function with signature `WITHDRAW_WITH_AUTHORIZATION_TYPEHASH()` and selector `0x2004d291`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "WITHDRAW_WITH_AUTHORIZATION_TYPEHASH",
        abi = "WITHDRAW_WITH_AUTHORIZATION_TYPEHASH()"
    )]
    pub struct WithdrawWithAuthorizationTypehashCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approveWithAuthorization` function with signature `approveWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0xe1560fd3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "approveWithAuthorization",
        abi = "approveWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct ApproveWithAuthorizationCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub valid_after: ::ethers::core::types::U256,
        pub valid_before: ::ethers::core::types::U256,
        pub nonce: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `authorizationState` function with signature `authorizationState(address,bytes32)` and selector `0xe94a0102`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "authorizationState", abi = "authorizationState(address,bytes32)")]
    pub struct AuthorizationStateCall {
        pub authorizer: ::ethers::core::types::Address,
        pub nonce: [u8; 32],
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `blacklist` function with signature `blacklist(address)` and selector `0xf9f92be4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "blacklist", abi = "blacklist(address)")]
    pub struct BlacklistCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `blacklisters` function with signature `blacklisters()` and selector `0x7cfa1cfa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "blacklisters", abi = "blacklisters()")]
    pub struct BlacklistersCall;
    ///Container type for all input parameters for the `cancelAuthorization` function with signature `cancelAuthorization(address,bytes32,uint8,bytes32,bytes32)` and selector `0x5a049a70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "cancelAuthorization",
        abi = "cancelAuthorization(address,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct CancelAuthorizationCall {
        pub authorizer: ::ethers::core::types::Address,
        pub nonce: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decreaseAllowanceWithAuthorization` function with signature `decreaseAllowanceWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0xb0086095`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "decreaseAllowanceWithAuthorization",
        abi = "decreaseAllowanceWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct DecreaseAllowanceWithAuthorizationCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub decrement: ::ethers::core::types::U256,
        pub valid_after: ::ethers::core::types::U256,
        pub valid_before: ::ethers::core::types::U256,
        pub nonce: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,bytes)` and selector `0xcf2c52cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,bytes)")]
    pub struct DepositCall {
        pub user: ::ethers::core::types::Address,
        pub deposit_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeMetaTransaction` function with signature `executeMetaTransaction(address,bytes,bytes32,bytes32,uint8)` and selector `0x0c53c51c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeMetaTransaction",
        abi = "executeMetaTransaction(address,bytes,bytes32,bytes32,uint8)"
    )]
    pub struct ExecuteMetaTransactionCall {
        pub user_address: ::ethers::core::types::Address,
        pub function_signature: ::ethers::core::types::Bytes,
        pub sig_r: [u8; 32],
        pub sig_s: [u8; 32],
        pub sig_v: u8,
    }
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `increaseAllowanceWithAuthorization` function with signature `increaseAllowanceWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0xc237108f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "increaseAllowanceWithAuthorization",
        abi = "increaseAllowanceWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct IncreaseAllowanceWithAuthorizationCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub increment: ::ethers::core::types::U256,
        pub valid_after: ::ethers::core::types::U256,
        pub valid_before: ::ethers::core::types::U256,
        pub nonce: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,string,uint8,address)` and selector `0xde7ea79d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(string,string,uint8,address)")]
    pub struct InitializeCall {
        pub new_name: ::std::string::String,
        pub new_symbol: ::std::string::String,
        pub new_decimals: u8,
        pub child_chain_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `0x158ef93e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    ///Container type for all input parameters for the `isBlacklisted` function with signature `isBlacklisted(address)` and selector `0xfe575a87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isBlacklisted", abi = "isBlacklisted(address)")]
    pub struct IsBlacklistedCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pausers` function with signature `pausers()` and selector `0xe8d6fc47`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pausers", abi = "pausers()")]
    pub struct PausersCall;
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rescueERC20` function with signature `rescueERC20(address,address,uint256)` and selector `0xb2118a8d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rescueERC20", abi = "rescueERC20(address,address,uint256)")]
    pub struct RescueERC20Call {
        pub token_contract: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rescuers` function with signature `rescuers()` and selector `0x100a1560`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rescuers", abi = "rescuers()")]
    pub struct RescuersCall;
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferWithAuthorization` function with signature `transferWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0xe3ee160e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "transferWithAuthorization",
        abi = "transferWithAuthorization(address,address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct TransferWithAuthorizationCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub valid_after: ::ethers::core::types::U256,
        pub valid_before: ::ethers::core::types::U256,
        pub nonce: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `unBlacklist` function with signature `unBlacklist(address)` and selector `0x1a895266`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unBlacklist", abi = "unBlacklist(address)")]
    pub struct UnBlacklistCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `updateMetadata` function with signature `updateMetadata(string,string)` and selector `0x56ce0e6c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateMetadata", abi = "updateMetadata(string,string)")]
    pub struct UpdateMetadataCall {
        pub new_name: ::std::string::String,
        pub new_symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawWithAuthorization` function with signature `withdrawWithAuthorization(address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)` and selector `0x7b766e5f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawWithAuthorization",
        abi = "withdrawWithAuthorization(address,uint256,uint256,uint256,bytes32,uint8,bytes32,bytes32)"
    )]
    pub struct WithdrawWithAuthorizationCall {
        pub owner: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub valid_after: ::ethers::core::types::U256,
        pub valid_before: ::ethers::core::types::U256,
        pub nonce: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ERC20Calls {
        ApproveWithAuthorizationTypehash(ApproveWithAuthorizationTypehashCall),
        BlacklisterRole(BlacklisterRoleCall),
        CancelAuthorizationTypehash(CancelAuthorizationTypehashCall),
        DecreaseAllowanceWithAuthorizationTypehash(
            DecreaseAllowanceWithAuthorizationTypehashCall,
        ),
        DefaultAdminRole(DefaultAdminRoleCall),
        DepositorRole(DepositorRoleCall),
        DomainSeparator(DomainSeparatorCall),
        Eip712Version(Eip712VersionCall),
        IncreaseAllowanceWithAuthorizationTypehash(
            IncreaseAllowanceWithAuthorizationTypehashCall,
        ),
        MetaTransactionTypehash(MetaTransactionTypehashCall),
        PauserRole(PauserRoleCall),
        PermitTypehash(PermitTypehashCall),
        RescuerRole(RescuerRoleCall),
        TransferWithAuthorizationTypehash(TransferWithAuthorizationTypehashCall),
        WithdrawWithAuthorizationTypehash(WithdrawWithAuthorizationTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveWithAuthorization(ApproveWithAuthorizationCall),
        AuthorizationState(AuthorizationStateCall),
        BalanceOf(BalanceOfCall),
        Blacklist(BlacklistCall),
        Blacklisters(BlacklistersCall),
        CancelAuthorization(CancelAuthorizationCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DecreaseAllowanceWithAuthorization(DecreaseAllowanceWithAuthorizationCall),
        Deposit(DepositCall),
        ExecuteMetaTransaction(ExecuteMetaTransactionCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        IncreaseAllowanceWithAuthorization(IncreaseAllowanceWithAuthorizationCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsBlacklisted(IsBlacklistedCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Pausers(PausersCall),
        Permit(PermitCall),
        RenounceRole(RenounceRoleCall),
        RescueERC20(RescueERC20Call),
        Rescuers(RescuersCall),
        RevokeRole(RevokeRoleCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferWithAuthorization(TransferWithAuthorizationCall),
        UnBlacklist(UnBlacklistCall),
        Unpause(UnpauseCall),
        UpdateMetadata(UpdateMetadataCall),
        Withdraw(WithdrawCall),
        WithdrawWithAuthorization(WithdrawWithAuthorizationCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ApproveWithAuthorizationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ApproveWithAuthorizationTypehash(decoded));
            }
            if let Ok(decoded)
                = <BlacklisterRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BlacklisterRole(decoded));
            }
            if let Ok(decoded)
                = <CancelAuthorizationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelAuthorizationTypehash(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceWithAuthorizationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowanceWithAuthorizationTypehash(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <DepositorRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositorRole(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <Eip712VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Version(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceWithAuthorizationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowanceWithAuthorizationTypehash(decoded));
            }
            if let Ok(decoded)
                = <MetaTransactionTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MetaTransactionTypehash(decoded));
            }
            if let Ok(decoded)
                = <PauserRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauserRole(decoded));
            }
            if let Ok(decoded)
                = <PermitTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PermitTypehash(decoded));
            }
            if let Ok(decoded)
                = <RescuerRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RescuerRole(decoded));
            }
            if let Ok(decoded)
                = <TransferWithAuthorizationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferWithAuthorizationTypehash(decoded));
            }
            if let Ok(decoded)
                = <WithdrawWithAuthorizationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawWithAuthorizationTypehash(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <ApproveWithAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ApproveWithAuthorization(decoded));
            }
            if let Ok(decoded)
                = <AuthorizationStateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AuthorizationState(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BlacklistCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Blacklist(decoded));
            }
            if let Ok(decoded)
                = <BlacklistersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Blacklisters(decoded));
            }
            if let Ok(decoded)
                = <CancelAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelAuthorization(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceWithAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowanceWithAuthorization(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <ExecuteMetaTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteMetaTransaction(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded)
                = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceWithAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowanceWithAuthorization(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <InitializedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialized(decoded));
            }
            if let Ok(decoded)
                = <IsBlacklistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBlacklisted(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded)
                = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded)
                = <PausersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pausers(decoded));
            }
            if let Ok(decoded)
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RescueERC20Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RescueERC20(decoded));
            }
            if let Ok(decoded)
                = <RescuersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rescuers(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferWithAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferWithAuthorization(decoded));
            }
            if let Ok(decoded)
                = <UnBlacklistCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnBlacklist(decoded));
            }
            if let Ok(decoded)
                = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded)
                = <UpdateMetadataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateMetadata(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <WithdrawWithAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawWithAuthorization(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApproveWithAuthorizationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlacklisterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelAuthorizationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowanceWithAuthorizationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Version(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowanceWithAuthorizationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MetaTransactionTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauserRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescuerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferWithAuthorizationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawWithAuthorizationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveWithAuthorization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AuthorizationState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Blacklist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Blacklisters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelAuthorization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowanceWithAuthorization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteMetaTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowanceWithAuthorization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBlacklisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pausers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescueERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rescuers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferWithAuthorization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnBlacklist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawWithAuthorization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveWithAuthorizationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlacklisterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelAuthorizationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseAllowanceWithAuthorizationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowanceWithAuthorizationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MetaTransactionTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PauserRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescuerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferWithAuthorizationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawWithAuthorizationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveWithAuthorization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AuthorizationState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Blacklist(element) => ::core::fmt::Display::fmt(element, f),
                Self::Blacklisters(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelAuthorization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowanceWithAuthorization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteMetaTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowanceWithAuthorization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsBlacklisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pausers(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescueERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rescuers(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferWithAuthorization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnBlacklist(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawWithAuthorization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApproveWithAuthorizationTypehashCall> for ERC20Calls {
        fn from(value: ApproveWithAuthorizationTypehashCall) -> Self {
            Self::ApproveWithAuthorizationTypehash(value)
        }
    }
    impl ::core::convert::From<BlacklisterRoleCall> for ERC20Calls {
        fn from(value: BlacklisterRoleCall) -> Self {
            Self::BlacklisterRole(value)
        }
    }
    impl ::core::convert::From<CancelAuthorizationTypehashCall> for ERC20Calls {
        fn from(value: CancelAuthorizationTypehashCall) -> Self {
            Self::CancelAuthorizationTypehash(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceWithAuthorizationTypehashCall>
    for ERC20Calls {
        fn from(value: DecreaseAllowanceWithAuthorizationTypehashCall) -> Self {
            Self::DecreaseAllowanceWithAuthorizationTypehash(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ERC20Calls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<DepositorRoleCall> for ERC20Calls {
        fn from(value: DepositorRoleCall) -> Self {
            Self::DepositorRole(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for ERC20Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<Eip712VersionCall> for ERC20Calls {
        fn from(value: Eip712VersionCall) -> Self {
            Self::Eip712Version(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceWithAuthorizationTypehashCall>
    for ERC20Calls {
        fn from(value: IncreaseAllowanceWithAuthorizationTypehashCall) -> Self {
            Self::IncreaseAllowanceWithAuthorizationTypehash(value)
        }
    }
    impl ::core::convert::From<MetaTransactionTypehashCall> for ERC20Calls {
        fn from(value: MetaTransactionTypehashCall) -> Self {
            Self::MetaTransactionTypehash(value)
        }
    }
    impl ::core::convert::From<PauserRoleCall> for ERC20Calls {
        fn from(value: PauserRoleCall) -> Self {
            Self::PauserRole(value)
        }
    }
    impl ::core::convert::From<PermitTypehashCall> for ERC20Calls {
        fn from(value: PermitTypehashCall) -> Self {
            Self::PermitTypehash(value)
        }
    }
    impl ::core::convert::From<RescuerRoleCall> for ERC20Calls {
        fn from(value: RescuerRoleCall) -> Self {
            Self::RescuerRole(value)
        }
    }
    impl ::core::convert::From<TransferWithAuthorizationTypehashCall> for ERC20Calls {
        fn from(value: TransferWithAuthorizationTypehashCall) -> Self {
            Self::TransferWithAuthorizationTypehash(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAuthorizationTypehashCall> for ERC20Calls {
        fn from(value: WithdrawWithAuthorizationTypehashCall) -> Self {
            Self::WithdrawWithAuthorizationTypehash(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for ERC20Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ERC20Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<ApproveWithAuthorizationCall> for ERC20Calls {
        fn from(value: ApproveWithAuthorizationCall) -> Self {
            Self::ApproveWithAuthorization(value)
        }
    }
    impl ::core::convert::From<AuthorizationStateCall> for ERC20Calls {
        fn from(value: AuthorizationStateCall) -> Self {
            Self::AuthorizationState(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC20Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BlacklistCall> for ERC20Calls {
        fn from(value: BlacklistCall) -> Self {
            Self::Blacklist(value)
        }
    }
    impl ::core::convert::From<BlacklistersCall> for ERC20Calls {
        fn from(value: BlacklistersCall) -> Self {
            Self::Blacklisters(value)
        }
    }
    impl ::core::convert::From<CancelAuthorizationCall> for ERC20Calls {
        fn from(value: CancelAuthorizationCall) -> Self {
            Self::CancelAuthorization(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for ERC20Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for ERC20Calls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceWithAuthorizationCall> for ERC20Calls {
        fn from(value: DecreaseAllowanceWithAuthorizationCall) -> Self {
            Self::DecreaseAllowanceWithAuthorization(value)
        }
    }
    impl ::core::convert::From<DepositCall> for ERC20Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExecuteMetaTransactionCall> for ERC20Calls {
        fn from(value: ExecuteMetaTransactionCall) -> Self {
            Self::ExecuteMetaTransaction(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ERC20Calls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for ERC20Calls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for ERC20Calls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ERC20Calls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ERC20Calls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for ERC20Calls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceWithAuthorizationCall> for ERC20Calls {
        fn from(value: IncreaseAllowanceWithAuthorizationCall) -> Self {
            Self::IncreaseAllowanceWithAuthorization(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ERC20Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedCall> for ERC20Calls {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<IsBlacklistedCall> for ERC20Calls {
        fn from(value: IsBlacklistedCall) -> Self {
            Self::IsBlacklisted(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC20Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for ERC20Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PauseCall> for ERC20Calls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for ERC20Calls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PausersCall> for ERC20Calls {
        fn from(value: PausersCall) -> Self {
            Self::Pausers(value)
        }
    }
    impl ::core::convert::From<PermitCall> for ERC20Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ERC20Calls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RescueERC20Call> for ERC20Calls {
        fn from(value: RescueERC20Call) -> Self {
            Self::RescueERC20(value)
        }
    }
    impl ::core::convert::From<RescuersCall> for ERC20Calls {
        fn from(value: RescuersCall) -> Self {
            Self::Rescuers(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ERC20Calls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ERC20Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ERC20Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for ERC20Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ERC20Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferWithAuthorizationCall> for ERC20Calls {
        fn from(value: TransferWithAuthorizationCall) -> Self {
            Self::TransferWithAuthorization(value)
        }
    }
    impl ::core::convert::From<UnBlacklistCall> for ERC20Calls {
        fn from(value: UnBlacklistCall) -> Self {
            Self::UnBlacklist(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for ERC20Calls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateMetadataCall> for ERC20Calls {
        fn from(value: UpdateMetadataCall) -> Self {
            Self::UpdateMetadata(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for ERC20Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAuthorizationCall> for ERC20Calls {
        fn from(value: WithdrawWithAuthorizationCall) -> Self {
            Self::WithdrawWithAuthorization(value)
        }
    }
    ///Container type for all return fields from the `APPROVE_WITH_AUTHORIZATION_TYPEHASH` function with signature `APPROVE_WITH_AUTHORIZATION_TYPEHASH()` and selector `0x10c8b404`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ApproveWithAuthorizationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `BLACKLISTER_ROLE` function with signature `BLACKLISTER_ROLE()` and selector `0xf515e6f2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BlacklisterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CANCEL_AUTHORIZATION_TYPEHASH` function with signature `CANCEL_AUTHORIZATION_TYPEHASH()` and selector `0xd9169487`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CancelAuthorizationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH` function with signature `DECREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH()` and selector `0x4b71f5c3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecreaseAllowanceWithAuthorizationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEPOSITOR_ROLE` function with signature `DEPOSITOR_ROLE()` and selector `0xa3b0b5a3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DepositorRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `EIP712_VERSION` function with signature `EIP712_VERSION()` and selector `0xeccec5a8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Eip712VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH` function with signature `INCREASE_ALLOWANCE_WITH_AUTHORIZATION_TYPEHASH()` and selector `0xc7b84f8d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IncreaseAllowanceWithAuthorizationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `META_TRANSACTION_TYPEHASH` function with signature `META_TRANSACTION_TYPEHASH()` and selector `0xfa2c322d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MetaTransactionTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PauserRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PermitTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `RESCUER_ROLE` function with signature `RESCUER_ROLE()` and selector `0x71372544`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RescuerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TRANSFER_WITH_AUTHORIZATION_TYPEHASH` function with signature `TRANSFER_WITH_AUTHORIZATION_TYPEHASH()` and selector `0xa0cc6a68`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferWithAuthorizationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `WITHDRAW_WITH_AUTHORIZATION_TYPEHASH` function with signature `WITHDRAW_WITH_AUTHORIZATION_TYPEHASH()` and selector `0x2004d291`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WithdrawWithAuthorizationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `authorizationState` function with signature `authorizationState(address,bytes32)` and selector `0xe94a0102`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AuthorizationStateReturn(pub u8);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `blacklisters` function with signature `blacklisters()` and selector `0x7cfa1cfa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BlacklistersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `executeMetaTransaction` function with signature `executeMetaTransaction(address,bytes,bytes32,bytes32,uint8)` and selector `0x0c53c51c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExecuteMetaTransactionReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `initialized` function with signature `initialized()` and selector `0x158ef93e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct InitializedReturn(pub bool);
    ///Container type for all return fields from the `isBlacklisted` function with signature `isBlacklisted(address)` and selector `0xfe575a87`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsBlacklistedReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `pausers` function with signature `pausers()` and selector `0xe8d6fc47`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PausersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `rescuers` function with signature `rescuers()` and selector `0x100a1560`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RescuersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferFromReturn(pub bool);
}

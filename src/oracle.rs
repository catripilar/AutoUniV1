pub use oracle::*;
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
pub mod oracle {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_accessController\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"int256\",\"name\":\"current\",\"type\":\"int256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"}],\"name\":\"AnswerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"startedBy\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"}],\"name\":\"NewRound\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"name\":\"OwnershipTransferRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"acceptOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"accessController\",\"outputs\":[{\"internalType\":\"contract AccessControllerInterface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"aggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\"}],\"name\":\"confirmAggregator\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\"}],\"name\":\"getAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\"}],\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\"},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\"}],\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestRound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\"},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\"}],\"name\":\"phaseAggregators\",\"outputs\":[{\"internalType\":\"contract AggregatorV2V3Interface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"phaseId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\"}],\"name\":\"proposeAggregator\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proposedAggregator\",\"outputs\":[{\"internalType\":\"contract AggregatorV2V3Interface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\"}],\"name\":\"proposedGetRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\"},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proposedLatestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\"},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_accessController\",\"type\":\"address\"}],\"name\":\"setController\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]\n";
    ///The parsed JSON ABI of the contract.
    pub static ORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct oracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for oracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for oracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for oracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for oracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(oracle)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> oracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accessController` (0xbc43cbaf) function
        pub fn access_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([188, 67, 203, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregator` (0x245a7bfc) function
        pub fn aggregator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([36, 90, 123, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `confirmAggregator` (0xa928c096) function
        pub fn confirm_aggregator(
            &self,
            aggregator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 40, 192, 150], aggregator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `description` (0x7284e416) function
        pub fn description(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAnswer` (0xb5ab58dc) function
        pub fn get_answer(
            &self,
            round_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([181, 171, 88, 220], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundData` (0x9a6fc8f5) function
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimestamp` (0xb633620c) function
        pub fn get_timestamp(
            &self,
            round_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 51, 98, 12], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestAnswer` (0x50d25bcd) function
        pub fn latest_answer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([80, 210, 91, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRound` (0x668a0f02) function
        pub fn latest_round(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 138, 15, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRoundData` (0xfeaf968c) function
        pub fn latest_round_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestTimestamp` (0x8205bf6a) function
        pub fn latest_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([130, 5, 191, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `phaseAggregators` (0xc1597304) function
        pub fn phase_aggregators(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([193, 89, 115, 4], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `phaseId` (0x58303b10) function
        pub fn phase_id(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([88, 48, 59, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposeAggregator` (0xf8a2abd3) function
        pub fn propose_aggregator(
            &self,
            aggregator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 162, 171, 211], aggregator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposedAggregator` (0xe8c4be30) function
        pub fn proposed_aggregator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([232, 196, 190, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposedGetRoundData` (0x6001ac53) function
        pub fn proposed_get_round_data(
            &self,
            round_id: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([96, 1, 172, 83], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposedLatestRoundData` (0x8f6b4d91) function
        pub fn proposed_latest_round_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([143, 107, 77, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setController` (0x92eefe9b) function
        pub fn set_controller(
            &self,
            access_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 238, 254, 155], access_controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AnswerUpdated` event
        pub fn answer_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AnswerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewRound` event
        pub fn new_round_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewRoundFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferRequested` event
        pub fn ownership_transfer_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferRequestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, oracleEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for oracle<M> {
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
    #[ethevent(name = "AnswerUpdated", abi = "AnswerUpdated(int256,uint256,uint256)")]
    pub struct AnswerUpdatedFilter {
        #[ethevent(indexed)]
        pub current: ::ethers::core::types::I256,
        #[ethevent(indexed)]
        pub round_id: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
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
    #[ethevent(name = "NewRound", abi = "NewRound(uint256,address,uint256)")]
    pub struct NewRoundFilter {
        #[ethevent(indexed)]
        pub round_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub started_by: ::ethers::core::types::Address,
        pub started_at: ::ethers::core::types::U256,
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
        name = "OwnershipTransferRequested",
        abi = "OwnershipTransferRequested(address,address)"
    )]
    pub struct OwnershipTransferRequestedFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum oracleEvents {
        AnswerUpdatedFilter(AnswerUpdatedFilter),
        NewRoundFilter(NewRoundFilter),
        OwnershipTransferRequestedFilter(OwnershipTransferRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for oracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
                return Ok(oracleEvents::AnswerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NewRoundFilter::decode_log(log) {
                return Ok(oracleEvents::NewRoundFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferRequestedFilter::decode_log(log) {
                return Ok(oracleEvents::OwnershipTransferRequestedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(oracleEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for oracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AnswerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewRoundFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AnswerUpdatedFilter> for oracleEvents {
        fn from(value: AnswerUpdatedFilter) -> Self {
            Self::AnswerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NewRoundFilter> for oracleEvents {
        fn from(value: NewRoundFilter) -> Self {
            Self::NewRoundFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferRequestedFilter> for oracleEvents {
        fn from(value: OwnershipTransferRequestedFilter) -> Self {
            Self::OwnershipTransferRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for oracleEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `accessController` function with signature `accessController()` and selector `0xbc43cbaf`
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
    #[ethcall(name = "accessController", abi = "accessController()")]
    pub struct AccessControllerCall;
    ///Container type for all input parameters for the `aggregator` function with signature `aggregator()` and selector `0x245a7bfc`
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
    #[ethcall(name = "aggregator", abi = "aggregator()")]
    pub struct AggregatorCall;
    ///Container type for all input parameters for the `confirmAggregator` function with signature `confirmAggregator(address)` and selector `0xa928c096`
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
    #[ethcall(name = "confirmAggregator", abi = "confirmAggregator(address)")]
    pub struct ConfirmAggregatorCall {
        pub aggregator: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `description` function with signature `description()` and selector `0x7284e416`
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
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    ///Container type for all input parameters for the `getAnswer` function with signature `getAnswer(uint256)` and selector `0xb5ab58dc`
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
    #[ethcall(name = "getAnswer", abi = "getAnswer(uint256)")]
    pub struct GetAnswerCall {
        pub round_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
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
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    ///Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `0xb633620c`
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
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(uint256)")]
    pub struct GetTimestampCall {
        pub round_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `latestAnswer` function with signature `latestAnswer()` and selector `0x50d25bcd`
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
    #[ethcall(name = "latestAnswer", abi = "latestAnswer()")]
    pub struct LatestAnswerCall;
    ///Container type for all input parameters for the `latestRound` function with signature `latestRound()` and selector `0x668a0f02`
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
    #[ethcall(name = "latestRound", abi = "latestRound()")]
    pub struct LatestRoundCall;
    ///Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
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
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    ///Container type for all input parameters for the `latestTimestamp` function with signature `latestTimestamp()` and selector `0x8205bf6a`
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
    #[ethcall(name = "latestTimestamp", abi = "latestTimestamp()")]
    pub struct LatestTimestampCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `phaseAggregators` function with signature `phaseAggregators(uint16)` and selector `0xc1597304`
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
    #[ethcall(name = "phaseAggregators", abi = "phaseAggregators(uint16)")]
    pub struct PhaseAggregatorsCall(pub u16);
    ///Container type for all input parameters for the `phaseId` function with signature `phaseId()` and selector `0x58303b10`
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
    #[ethcall(name = "phaseId", abi = "phaseId()")]
    pub struct PhaseIdCall;
    ///Container type for all input parameters for the `proposeAggregator` function with signature `proposeAggregator(address)` and selector `0xf8a2abd3`
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
    #[ethcall(name = "proposeAggregator", abi = "proposeAggregator(address)")]
    pub struct ProposeAggregatorCall {
        pub aggregator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `proposedAggregator` function with signature `proposedAggregator()` and selector `0xe8c4be30`
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
    #[ethcall(name = "proposedAggregator", abi = "proposedAggregator()")]
    pub struct ProposedAggregatorCall;
    ///Container type for all input parameters for the `proposedGetRoundData` function with signature `proposedGetRoundData(uint80)` and selector `0x6001ac53`
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
    #[ethcall(name = "proposedGetRoundData", abi = "proposedGetRoundData(uint80)")]
    pub struct ProposedGetRoundDataCall {
        pub round_id: u128,
    }
    ///Container type for all input parameters for the `proposedLatestRoundData` function with signature `proposedLatestRoundData()` and selector `0x8f6b4d91`
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
    #[ethcall(name = "proposedLatestRoundData", abi = "proposedLatestRoundData()")]
    pub struct ProposedLatestRoundDataCall;
    ///Container type for all input parameters for the `setController` function with signature `setController(address)` and selector `0x92eefe9b`
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
    #[ethcall(name = "setController", abi = "setController(address)")]
    pub struct SetControllerCall {
        pub access_controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum oracleCalls {
        AcceptOwnership(AcceptOwnershipCall),
        AccessController(AccessControllerCall),
        Aggregator(AggregatorCall),
        ConfirmAggregator(ConfirmAggregatorCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetAnswer(GetAnswerCall),
        GetRoundData(GetRoundDataCall),
        GetTimestamp(GetTimestampCall),
        LatestAnswer(LatestAnswerCall),
        LatestRound(LatestRoundCall),
        LatestRoundData(LatestRoundDataCall),
        LatestTimestamp(LatestTimestampCall),
        Owner(OwnerCall),
        PhaseAggregators(PhaseAggregatorsCall),
        PhaseId(PhaseIdCall),
        ProposeAggregator(ProposeAggregatorCall),
        ProposedAggregator(ProposedAggregatorCall),
        ProposedGetRoundData(ProposedGetRoundDataCall),
        ProposedLatestRoundData(ProposedLatestRoundDataCall),
        SetController(SetControllerCall),
        TransferOwnership(TransferOwnershipCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for oracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded)
                = <AccessControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AccessController(decoded));
            }
            if let Ok(decoded)
                = <AggregatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Aggregator(decoded));
            }
            if let Ok(decoded)
                = <ConfirmAggregatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfirmAggregator(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Description(decoded));
            }
            if let Ok(decoded)
                = <GetAnswerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAnswer(decoded));
            }
            if let Ok(decoded)
                = <GetRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoundData(decoded));
            }
            if let Ok(decoded)
                = <GetTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTimestamp(decoded));
            }
            if let Ok(decoded)
                = <LatestAnswerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestAnswer(decoded));
            }
            if let Ok(decoded)
                = <LatestRoundCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestRound(decoded));
            }
            if let Ok(decoded)
                = <LatestRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestRoundData(decoded));
            }
            if let Ok(decoded)
                = <LatestTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestTimestamp(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PhaseAggregatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PhaseAggregators(decoded));
            }
            if let Ok(decoded)
                = <PhaseIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PhaseId(decoded));
            }
            if let Ok(decoded)
                = <ProposeAggregatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposeAggregator(decoded));
            }
            if let Ok(decoded)
                = <ProposedAggregatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposedAggregator(decoded));
            }
            if let Ok(decoded)
                = <ProposedGetRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposedGetRoundData(decoded));
            }
            if let Ok(decoded)
                = <ProposedLatestRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposedLatestRoundData(decoded));
            }
            if let Ok(decoded)
                = <SetControllerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetController(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for oracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccessController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Aggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfirmAggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Description(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAnswer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestAnswer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PhaseAggregators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PhaseId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProposeAggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposedAggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposedGetRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposedLatestRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for oracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccessController(element) => ::core::fmt::Display::fmt(element, f),
                Self::Aggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfirmAggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Description(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PhaseAggregators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PhaseId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeAggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposedAggregator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposedGetRoundData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposedLatestRoundData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetController(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for oracleCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<AccessControllerCall> for oracleCalls {
        fn from(value: AccessControllerCall) -> Self {
            Self::AccessController(value)
        }
    }
    impl ::core::convert::From<AggregatorCall> for oracleCalls {
        fn from(value: AggregatorCall) -> Self {
            Self::Aggregator(value)
        }
    }
    impl ::core::convert::From<ConfirmAggregatorCall> for oracleCalls {
        fn from(value: ConfirmAggregatorCall) -> Self {
            Self::ConfirmAggregator(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for oracleCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DescriptionCall> for oracleCalls {
        fn from(value: DescriptionCall) -> Self {
            Self::Description(value)
        }
    }
    impl ::core::convert::From<GetAnswerCall> for oracleCalls {
        fn from(value: GetAnswerCall) -> Self {
            Self::GetAnswer(value)
        }
    }
    impl ::core::convert::From<GetRoundDataCall> for oracleCalls {
        fn from(value: GetRoundDataCall) -> Self {
            Self::GetRoundData(value)
        }
    }
    impl ::core::convert::From<GetTimestampCall> for oracleCalls {
        fn from(value: GetTimestampCall) -> Self {
            Self::GetTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestAnswerCall> for oracleCalls {
        fn from(value: LatestAnswerCall) -> Self {
            Self::LatestAnswer(value)
        }
    }
    impl ::core::convert::From<LatestRoundCall> for oracleCalls {
        fn from(value: LatestRoundCall) -> Self {
            Self::LatestRound(value)
        }
    }
    impl ::core::convert::From<LatestRoundDataCall> for oracleCalls {
        fn from(value: LatestRoundDataCall) -> Self {
            Self::LatestRoundData(value)
        }
    }
    impl ::core::convert::From<LatestTimestampCall> for oracleCalls {
        fn from(value: LatestTimestampCall) -> Self {
            Self::LatestTimestamp(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for oracleCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PhaseAggregatorsCall> for oracleCalls {
        fn from(value: PhaseAggregatorsCall) -> Self {
            Self::PhaseAggregators(value)
        }
    }
    impl ::core::convert::From<PhaseIdCall> for oracleCalls {
        fn from(value: PhaseIdCall) -> Self {
            Self::PhaseId(value)
        }
    }
    impl ::core::convert::From<ProposeAggregatorCall> for oracleCalls {
        fn from(value: ProposeAggregatorCall) -> Self {
            Self::ProposeAggregator(value)
        }
    }
    impl ::core::convert::From<ProposedAggregatorCall> for oracleCalls {
        fn from(value: ProposedAggregatorCall) -> Self {
            Self::ProposedAggregator(value)
        }
    }
    impl ::core::convert::From<ProposedGetRoundDataCall> for oracleCalls {
        fn from(value: ProposedGetRoundDataCall) -> Self {
            Self::ProposedGetRoundData(value)
        }
    }
    impl ::core::convert::From<ProposedLatestRoundDataCall> for oracleCalls {
        fn from(value: ProposedLatestRoundDataCall) -> Self {
            Self::ProposedLatestRoundData(value)
        }
    }
    impl ::core::convert::From<SetControllerCall> for oracleCalls {
        fn from(value: SetControllerCall) -> Self {
            Self::SetController(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for oracleCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<VersionCall> for oracleCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `accessController` function with signature `accessController()` and selector `0xbc43cbaf`
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
    pub struct AccessControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `aggregator` function with signature `aggregator()` and selector `0x245a7bfc`
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
    pub struct AggregatorReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `description` function with signature `description()` and selector `0x7284e416`
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
    pub struct DescriptionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getAnswer` function with signature `getAnswer(uint256)` and selector `0xb5ab58dc`
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
    pub struct GetAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
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
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `0xb633620c`
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
    pub struct GetTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestAnswer` function with signature `latestAnswer()` and selector `0x50d25bcd`
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
    pub struct LatestAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `latestRound` function with signature `latestRound()` and selector `0x668a0f02`
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
    pub struct LatestRoundReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
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
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `latestTimestamp` function with signature `latestTimestamp()` and selector `0x8205bf6a`
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
    pub struct LatestTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `phaseAggregators` function with signature `phaseAggregators(uint16)` and selector `0xc1597304`
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
    pub struct PhaseAggregatorsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `phaseId` function with signature `phaseId()` and selector `0x58303b10`
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
    pub struct PhaseIdReturn(pub u16);
    ///Container type for all return fields from the `proposedAggregator` function with signature `proposedAggregator()` and selector `0xe8c4be30`
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
    pub struct ProposedAggregatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proposedGetRoundData` function with signature `proposedGetRoundData(uint80)` and selector `0x6001ac53`
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
    pub struct ProposedGetRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `proposedLatestRoundData` function with signature `proposedLatestRoundData()` and selector `0x8f6b4d91`
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
    pub struct ProposedLatestRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::ethers::core::types::U256);
}

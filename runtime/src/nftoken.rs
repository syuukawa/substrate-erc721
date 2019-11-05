/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, ensure};
use system::ensure_signed;
// use support::dispatch::Vec;
// use codec::{Encode, Decode};
use rstd::result;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

//参考nf-token.sol
// This module's storage items.
// TODO 变量的名称开始大写“IdToOwner”
decl_storage! {
	trait Store for Module<T: Trait> as Nftoken {

		//TODO 范型使用<T>
		/**
		* @dev A mapping from NFT ID to the address that owns it.
		*/
		IdToOwner get(id_to_owner):map u128=>T::AccountId;

		/**
		* @dev Mapping from NFT ID to approved address.
		*/
		IdToApproval get(id_to_approval):map u128=>T::AccountId;

	   /**
		* @dev Mapping from owner address to count of his tokens.
		*/
		OwnerToNFTokenCount get(owner_to_nf_token_count):map T::AccountId=>u128;

		/**
		* @dev Mapping from owner address to mapping of operator addresses.
		*/
		// mapping (address => mapping (address => bool)) internal OwnerToOperators;
		OwnerToOperators get(owner_to_operators):map (T::AccountId,T::AccountId)=>bool;
	}
}


//TODOEvent中的函数以“，”结尾而不是“；”
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {

		/**
		* @dev Emits when ownership of any NFT changes by any mechanism. This event emits when NFTs are
		* created (`from` == 0) and destroyed (`to` == 0). Exception: during contract creation, any
		* number of NFTs may be created and assigned without emitting Transfer. At the time of any
		* transfer, the approved address for that NFT (if any) is reset to none.
		* @param _from Sender of NFT (if address is zero address it indicates token creation).
		* @param _to Receiver of NFT (if address is zero address it indicates token destruction).
		* @param _token_id The NFT that got transfered.
		*/
		Transfer(AccountId, AccountId, u128),

		/**
		* @dev This emits when the approved address for an NFT is changed or reaffirmed. The zero
		* address indicates there is no approved address. When a Transfer event emits, this also
		* indicates that the approved address for that NFT (if any) is reset to none.
		* @param _owner Owner of NFT.
		* @param _approved Address that we are approving.
		* @param _token_id NFT which we are approving.
		*/
		Approval(AccountId, AccountId, u128),

		/**
		* @dev This emits when an operator is enabled or disabled for an owner. The operator can manage
		* all NFTs of the owner.
		* @param _owner Owner of NFT.
		* @param _operator Address to which we are setting operator rights.
		* @param _approved Status of operator rights(true if operator rights are given and false if
		* revoked).
		*/
		ApprovalForAll(AccountId, AccountId, bool),

	}
);

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		// pub fn do_something(origin, something: u32) -> Result {
		// 	// TODO: You only need this if you want to check it was signed.
		// 	let who = ensure_signed(origin)?;

		// 	// TODO: Code to execute when something calls this.
		// 	// For example: the following line stores the passed in u32 in the storage
		// 	Something::put(something);

		// 	// here we are raising the Something event
		// 	Self::deposit_event(RawEvent::SomethingStored(something, who));
		// 	Ok(())
		// }
		// fn safeTransferFrom(_from: T::AccountId, _to: T::AccountId, _token_id: u128, _data: Vec[8]);

		// fn safeTransferFrom(_from: T::AccountId, _to: T::AccountId, _token_id: u128){
		// 	_safeTransferFrom(_from, _to, _token_id, "");
		// }

		/**
		* @dev Throws unless `msg.sender` is the current owner, an authorized operator, or the approved
		* address for this NFT. Throws if `_from` is not the current owner. Throws if `_to` is the zero
		* address. Throws if `_tokenId` is not a valid NFT. This function can be changed to payable.
		* @notice The caller is responsible to confirm that `_to` is capable of receiving NFTs or else
		* they maybe be permanently lost.
		* @param _from The current owner of the NFT.
		* @param _to The new owner.
		* @param _tokenId The NFT to transfer.
		*/
		fn transfer_from(origin, _to: T::AccountId, _token_id: u128) -> result::Result<(), &'static str> {
			
			let _from = ensure_signed(origin)?;

			ensure!(Self::_can_transfer(_from.clone(),_token_id), "Can not be transfered");

			ensure!(Self::_valid_nftoken(_token_id), "invalid NFToken");

			let _token_owner = <IdToOwner<T>>::get(_token_id);

			ensure!(_token_owner == _from, "not the tokenOwner");

		 	//TODO require(_to != address(0));

			Self::_transfer(_to, _token_id);

			 Ok(())
		}		

		/**
		* @dev Set or reaffirm the approved address for an NFT. This function can be changed to payable.
		* @notice The zero address indicates there is no approved address. Throws unless `msg.sender` is
		* the current NFT owner, or an authorized operator of the current owner.
		* @param _approved Address to be approved for the given NFT ID.
		* @param _tokenId ID of the token to be approved.
		*/
		fn approve(origin, _token_id:u128) -> result::Result<(), &'static str> {
			
			let _approved = ensure_signed(origin)?;

			let _token_owner = <IdToOwner<T>>::get(_token_id);
			if Self::_can_operate(_token_owner.clone(),_token_id) && Self::_valid_nftoken(_token_id){
				
			}
			ensure!(_approved.clone() != _token_owner.clone(),"");

			<IdToApproval<T>>::insert(_token_id,_approved.clone());

			//事件
			Self::deposit_event(RawEvent::Approval(_token_owner, _approved,_token_id));

			Ok(())

		}

		/**
		* @dev Enables or disables approval for a third party ("operator") to manage all of
		* `msg.sender`'s assets. It also emits the ApprovalForAll event.
		* @notice This works even if sender doesn't own any tokens at the time.
		* @param _operator Address to add to the set of authorized operators.
		* @param _approved True if the operators is approved, false to revoke approval.
		*/
		fn set_approval_for_all(origin,_operator: T::AccountId, _approved:bool){

			let _sender = ensure_signed(origin)?;

			<OwnerToOperators<T>>::insert((_sender.clone(),_operator.clone()),_approved);
			//事件
			Self::deposit_event(RawEvent::ApprovalForAll(_sender, _operator,_approved));
		}

		/**
		* Create a ERC721 Token
		*/	
		fn create(origin,_token_id:u128) -> result::Result<(), &'static str> {
			let _sender = ensure_signed(origin)?;
			Self::_mint(_sender,tokenid);
			Ok(())
		}
	}
}


impl<T: Trait> Module<T> {

	/**
	* modifier
	* @dev Guarantees that the msg.sender is an owner or operator of the given NFT.
	* @param _token_id ID of the NFT to validate.
	* 是拥有者获取获得了拥有者的授权才可以进行操作
	*/
	fn _can_operate(_origin:T::AccountId, _token_id: u128)-> bool {
		let _token_owner: T::AccountId = <IdToOwner<T>>::get(_token_id);
		if _token_owner == _origin || <OwnerToOperators<T>>::get((_token_owner,_origin)) {
			return true;
		} else {
		  	return false;
		}
	}

	/**
	* modifier
	* @dev Guarantees that the msg.sender is allowed to transfer NFT.
	* @param _token_id ID of the NFT to transfer.
	* 转移的条件，拥有者,拥有者授权，代理操作_
	*/
	fn _can_transfer(_origin:T::AccountId, _token_id:u128)->bool{
		let _token_owner = <IdToOwner<T>>::get(_token_id);
		if _token_owner == _origin || 
		 	<IdToApproval<T>>::get(_token_id) == _token_owner || 
			 	<OwnerToOperators<T>>::get((_token_owner,_origin)) {
			true
		} else{
			false
		}
	}

	/**
	* modifier
	* @dev Guarantees that _token_id is a valid Token.
	* @param _token_id ID of the NFT to validate.
	* 查询tokenId是否存在
	*/
	fn _valid_nftoken(_token_id:u128)->bool{
		<IdToOwner<T>>::exists(_token_id)
	}

	/**
	* @dev Returns the number of NFTs owned by `_owner`. NFTs assigned to the zero address are
	* considered invalid, and this function throws for queries about the zero address.
	* @param _owner Address for whom to query the balance.
	* @return Balance of _owner.
	* balanceOf
	*/
	pub fn _balance_of(_owner: T::AccountId)->u128{
		Self::_get_owner_nft_count(_owner)
	}

	/**
	* @dev Returns the address of the owner of the NFT. NFTs assigned to zero address are considered
	* invalid, and queries about them do throw.
	* @param _tokenId The identifier for an NFT.
	* @return Address of _tokenId owner.
	* ownerOf
	*/
	pub fn _owner_of(_token_id:u128)->T::AccountId{

		<IdToOwner<T>>::get(_token_id)
	}

	/**
	* getApproved
	* @dev Get the approved address for a single NFT.
	* @notice Throws if `_tokenId` is not a valid NFT.
	* @param _tokenId ID of the NFT to query the approval of.
	* @return Address that _tokenId is approved for. 
	*/
	pub fn _get_approved(_token_id:u128) -> result::Result<T::AccountId, &'static str>{

		ensure!(Self::_valid_nftoken(_token_id),"invalid tokenId") ;
		let account_id: T::AccountId = <IdToApproval<T>>::get(_token_id);
		Ok(account_id)
	}

	/**
	* @dev Checks if `_operator` is an approved operator for `_owner`.
	* @param _owner The address that owns the NFTs.
	* @param _operator The address that acts on behalf of the owner.
	* @return True if approved for all, false otherwise.
	*/
	pub fn _is_approved_for_all(_owner:T::AccountId, _operator:T::AccountId)->bool{
		<OwnerToOperators<T>>::get((_owner,_operator))
	}

  /**
   * @dev Removes a NFT from owner.
   * @notice Use and override this function with caution. Wrong usage can have serious consequences.
   * @param _from Address from wich we want to remove the NFT.
   * @param _tokenId Which NFT we want to remove.
   * owner和授权处理
   */
	fn _remove_nftoken(_from:T::AccountId,_token_id:u128)-> result::Result<(), &'static str> {
		
		let _id_owner = <IdToOwner<T>>::get(_token_id);

		ensure!(_from == _id_owner, "not the owner");

		let _owner_count = <OwnerToNFTokenCount<T>>::get(_from.clone());
		let _owner_count_u128 = _owner_count.checked_sub(1).unwrap();
		<OwnerToNFTokenCount<T>>::insert(_from,_owner_count_u128);

		<IdToOwner<T>>::remove(_token_id);
		
		Ok(())
	}

  /**
   * @dev Assignes a new NFT to owner.
   * @notice Use and override this function with caution. Wrong usage can have serious consequences.
   * @param _to Address to wich we want to add the NFT.
   * @param _tokenId Which NFT we want to add.
   */
	fn _add_nftoken(_to: T::AccountId, _token_id:u128){

		<IdToOwner<T>>::insert(_token_id,_to.clone());

		let _owner_count = <OwnerToNFTokenCount<T>>::get(_to.clone());
		let _owner_count_u128 = _owner_count.checked_add(1).unwrap();
		<OwnerToNFTokenCount<T>>::insert(_to,_owner_count_u128);

		//<TotalSupply<T>>::put(token_id + 1.into());
		//<OwnerToTokenList<T>>::append(owner, token_id); 

	}

	/**
	* @dev Helper function that gets NFT count of owner. This is needed for overriding in enumerable
	* extension to remove double storage (gas optimization) of owner nft count.
	* @param _owner Address for whom to query the count.
	* @return Number of _owner NFTs.
	*/
	fn _get_owner_nft_count(_owner:T::AccountId) -> u128{
		<OwnerToNFTokenCount<T>>::get(_owner)
	}

	// _safeTransferFrom

	/** 
	* @dev Clears the current approval of a given NFT ID.
	* @param _tokenId ID of the NFT to be transferred.
	*/
	fn _clear_approval(_token_id:u128){
		<IdToApproval<T>>::remove(_token_id);
	}

	//===========================
	/**
	* @dev Actually preforms the transfer.
	* @notice Does NO checks.
	* @param _to Address of a new owner.
	* @param _token_id The NFT that is being transferred.
	*/
	fn _transfer(_to:T::AccountId, _token_id:u128) -> result::Result<(), &'static str>{
	
		let _from = <IdToOwner<T>>::get(_token_id);
		//idToApproval[_tokenId] != address(0)
		Self::_clear_approval(_token_id);

		Self::_remove_nftoken(_from.clone(),_token_id);

		Self::_add_nftoken(_to.clone(),_token_id);

		//事件
		Self::deposit_event(RawEvent::Transfer(_from, _to,_token_id));

		Ok(())
	}

  	/**
	* @dev Mints a new NFT.
	* @notice This is an internal function which should be called from user-implemented external
	* mint function. Its purpose is to show and properly initialize data structures when using this
	* implementation.
	* @param _to The address that will own the minted NFT.
	* @param _tokenId of the NFT to be minted by the msg.sender.
	*/
	fn _mint(_to:T::AccountId,_token_id:u128) -> result::Result<(), &'static str> {
		
		//Token数量的判断

		// let _id_owner = <IdToOwner<T>>::exists(_to.clone());
		// ensure!(_id_owner,"_token_id has existed");

		Self::_add_nftoken(_to.clone(),_token_id);

		//事件
		//TODO AccountId 怎么传空值
		Self::deposit_event(RawEvent::Transfer(_to.clone(), _to.clone(),_token_id));

		Ok(())
	}

  /**
   * @dev Burns a NFT.
   * @notice This is an internal function which should be called from user-implemented external burn
   * function. Its purpose is to show and properly initialize data structures when using this
   * implementation. Also, note that this burn implementation allows the minter to re-mint a burned
   * NFT.
   * @param _tokenId ID of the NFT to be burned.
   */
	fn _burn(_token_id:u128) -> result::Result<(), &'static str>{

		ensure!(Self::_valid_nftoken(_token_id),"invalid tokenid");

		let _token_owner = <IdToOwner<T>>::get(_token_id);

		Self::_clear_approval(_token_id);

		Self::_remove_nftoken(_token_owner.clone(),_token_id);

		// let Option(remove_result) = Self::_remove_nftoken(_token_owner.clone(),_token_id);
		// //Option 错误处理
		// match remove_result {
		// 	Result::Ok(val) => val,
		// 	Result::Err(err) => return Result::Err(err);
		// 	// panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
		// }

		//事件
		//TODO AccountId 怎么传空值
		Self::deposit_event(RawEvent::Transfer(_token_owner.clone(), _token_owner.clone(),_token_id));

		Ok(())
	}
}
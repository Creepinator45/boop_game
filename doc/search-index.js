var searchIndex = JSON.parse('{\
"boop_game":{"doc":"","t":[13,4,4,3,13,3,4,13,13,13,13,3,13,13,13,13,13,13,13,13,3,3,3,3,13,4,3,4,3,3,13,4,4,13,3,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,5,12,12,12,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,12,12],"n":["Big","Cell","CellErrorKind","CheckCellError","CheckingOutOfBounds","Coordinate","CoordinateErrorKind","Empty","Empty","Empty","Empty","GameState","InvalidFormat","InvalidFormat","MissingPiece","OutOfBounds","OutOfBoundsX","OutOfBoundsX","OutOfBoundsY","OutOfBoundsY","ParseCoordinateError","ParsePiecePlacementError","ParseSizeError","Piece","Piece","PieceErrorKind","PiecePlacement","PiecePlacementErrorKind","PlacePieceError","Player","PositionOccupied","Size","SizeErrorKind","Small","ThreeInRow","UnknownValue","ValueErrorCoordinate","ValueErrorSize","ValueErrorX","ValueErrorY","__description","__description","__description","__description","__description","ask_player","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","check_board","check_cell","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","coordinate","display","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_str","from_str","from_str","game_board","init","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","kind","kind","kind","kind","kind","main","name","owner","piece_pool","place_piece","size","size","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","turn_count","turn_order","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","win","x","y"],"q":["boop_game","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[19,0,0,0,14,0,0,16,17,18,21,0,16,18,15,21,14,15,14,15,0,0,0,0,21,0,0,0,0,0,15,0,0,19,0,17,18,18,16,16,1,3,4,5,6,7,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,9,9,1,14,3,15,4,16,5,17,6,18,19,20,21,10,12,1,14,3,15,4,16,5,17,6,18,19,20,21,10,12,7,9,1,14,3,15,4,16,5,17,6,18,19,20,23,21,10,12,1,14,3,15,4,4,16,5,5,17,6,6,18,19,20,23,21,9,10,7,12,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,19,10,7,9,9,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,1,3,4,5,6,0,23,20,23,9,20,7,1,14,3,15,4,16,5,17,6,18,19,20,21,10,12,4,5,6,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,9,9,1,14,3,15,4,16,5,17,6,18,19,20,23,21,9,10,7,12,0,10,10],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[1,2],[3,2],[4,2],[5,2],[6,2],[2,[[8,[7,6]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[9,[11,[10]]]],[[9,10],[[8,[[13,[12]],1]]]],[1,1],[14,14],[3,3],[15,15],[4,4],[16,16],[5,5],[17,17],[6,6],[18,18],[19,19],[20,20],[21,21],[10,10],[12,12],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[9],[[1,1],22],[[14,14],22],[[3,3],22],[[15,15],22],[[4,4],22],[[16,16],22],[[5,5],22],[[17,17],22],[[6,6],22],[[18,18],22],[[19,19],22],[[20,20],22],[[23,23],22],[[21,21],22],[[10,10],22],[[12,12],22],[[1,24],25],[[14,24],25],[[3,24],25],[[15,24],25],[[4,24],25],[[4,24],25],[[16,24],25],[[5,24],25],[[5,24],25],[[17,24],25],[[6,24],25],[[6,24],25],[[18,24],25],[[19,24],25],[[20,24],25],[[23,24],25],[[21,24],25],[[9,24],25],[[10,24],25],[[7,24],25],[[12,24],25],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[2,[[8,[19]]]],[2,[[8,[10]]]],[2,[[8,[7]]]],0,[[],9],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,[[]],0,0,0,[[9,7],[[8,[3]]]],0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],26],[[],26],[[],26],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],0,0,[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[[],27],[2],0,0],"p":[[3,"CheckCellError"],[15,"str"],[3,"PlacePieceError"],[3,"ParseCoordinateError"],[3,"ParseSizeError"],[3,"ParsePiecePlacementError"],[3,"PiecePlacement"],[4,"Result"],[3,"GameState"],[3,"Coordinate"],[4,"Option"],[3,"ThreeInRow"],[3,"Vec"],[4,"CellErrorKind"],[4,"PieceErrorKind"],[4,"CoordinateErrorKind"],[4,"SizeErrorKind"],[4,"PiecePlacementErrorKind"],[4,"Size"],[3,"Piece"],[4,"Cell"],[15,"bool"],[3,"Player"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
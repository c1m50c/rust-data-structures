use super::{LinkedList, macros::list};


#[test]
fn push_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);
    assert_eq!(list, list![5, 4, 3, 2, 1]);
    assert_eq!(list.len(), 5);
}

#[test]
fn push_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    assert_eq!(list, list![1, 2, 3, 4, 5]);
    assert_eq!(list.len(), 5);
}

#[test]
fn append_list() {
    let mut list_one: LinkedList<&str> = list!["One", "Two", "Three"];
    let mut list_two: LinkedList<&str> = list!["Four", "Five", "Six"];
    list_one.append_list(&mut list_two);
    assert_eq!(list_one, list!["One", "Two", "Three", "Four", "Five", "Six"]);
    assert_eq!(list_one.len(), 6);
}

#[test]
fn remove() {
    let mut list: LinkedList<&str> = list!["One", "Two", "Three"];
    list.remove_back();
    assert_eq!(list, list!["One", "Two"]);
    list.remove_front();
    assert_eq!(list, list!["Two"]);
}

#[test]
fn pop() {
    let mut list: LinkedList<i32> = list![1, 2, 3, 4, 5];
    
    let pop_front = list.pop_front();
    assert_eq!(pop_front, Some(1));

    let pop_back = list.pop_back();
    assert_eq!(pop_back, Some(5));
    
    assert_eq!(list, list![2, 3, 4]);
}

#[test]
fn clear() {
    let mut list: LinkedList<i32> = list![6, 6, 6];
    list.clear();
    assert_eq!(list, LinkedList::<i32>::new());
}

#[test]
fn is_empty() {
    let mut list: LinkedList<&str> = list!["I", "am", "not", "empty."];
    assert_eq!(list.is_empty(), false);
    list.clear();
    assert_eq!(list.is_empty(), true);
}

#[test]
fn get() {
    let list: LinkedList<i32> = list![1, 2, 3, 4, 5];
    assert_eq!(list.get(2), Some(&3));
}

#[test]
fn get_mut() {
    let list: LinkedList<f32> = list![1.0, 2.0, 3.0, 4.0, 5.0];
    let got = list.get_mut(2).unwrap();
    assert_eq!(list.get_mut(2), Some(&mut 3.0));
    *got = 6.0;
    assert_eq!(list.get_mut(2), Some(&mut 6.0));
}

#[test]
fn search() {
    let list: LinkedList<&str> = list!["Search", "Idk", "Maybe", "This?"];
    assert_eq!(list.search("This?"), Some(3));
}

#[test]
fn eq() {
    let list1 = list![3, 2, 1];
    let list2 = list![3, 2, 1];
    assert_eq!(list1, list2);
}

#[test]
fn ne() {
    let list1 = list![4, 0, 4];
    let list2 = list![4, 0, 0];
    assert_ne!(list1, list2);
    
    let list1 = list![4, 0, 4, 0];
    assert_ne!(list1, list2);
}

#[test]
fn index() {
    let list: LinkedList<&str> = list!["Hey", "this", "is", "a", "Linked", "List"];
    assert_eq!(list[0], "Hey");
    assert_eq!(list[1], "this");
    assert_eq!(list[2], "is");
    assert_eq!(list[3], "a");
    assert_eq!(list[4], "Linked");
    assert_eq!(list[5], "List");
}

#[test]
fn index_mut() {
    let mut list: LinkedList<i32> = list![3, 3, 3];
    assert_eq!(list, list![3, 3, 3]);
    list[0] = 6;
    list[1] = 6;
    list[2] = 6;
    assert_eq!(list, list![6, 6, 6]);
}

#[test]
fn default() {
    let list: LinkedList<u8> = LinkedList::default();
    assert_eq!(list, LinkedList::new());
}

#[test]
fn length() {
    assert_eq!(list![1, 2, 3].len(), 3);
}

#[test]
fn display() {
    assert_eq!(format!("{}", list![1, 3, 3, 7]), "[1, 3, 3, 7]");
    assert_eq!(format!("{}", LinkedList::<i32>::new()), "[]");
}

#[test]
fn from_vec() {
    let list = LinkedList::from(vec![1, 2, 3]);
    assert_eq!(list, list![1, 2, 3]);
}

#[test]
fn from_slice() {
    let slice: &[i32] = &[1, 2, 3];
    let list = LinkedList::from(slice);
    assert_eq!(list, list![1, 2, 3]);
}

#[test]
fn from_array() {
    let arr: [i32; 3] = [1, 2, 3];
    let list = LinkedList::from(arr);
    assert_eq!(list, list![1, 2, 3]);
}

#[test]
fn from_str() {
    let x = "5 4 3 2 1";
    assert_eq!(x.parse::<LinkedList<i32>>().unwrap(), list![5, 4, 3, 2, 1]);
}

#[test]
fn as_vector() {
    let list = list![1, 3, 3, 7];
    assert_eq!(list.as_vector(), vec![1, 3, 3, 7]);
}

#[test]
fn insert() {
    let mut list = list![1, 1];
    list.insert(2, 1);
    assert_eq!(list, list![1, 2, 1]);
    assert_eq!(list.length, 3);
    assert_eq!(list[1], 2);
}


#[test]
fn iterator() {
    let mut sum = 0;

    for i in list![1, 3, 3, 7, 3, 0, 0, 5] {
        sum += i;
    }

    assert_eq!(sum, 22);
}
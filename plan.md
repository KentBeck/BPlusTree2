We are implementing a BPlusTreeMap in Rust. We are writing the code like Kent Beck would write it. Before modifying code we consider whether tidying first would make the change easier. Commits will be separated into commits that change the behavior of the code and commits that only change the structure of the code. Write the code one test at a time. Write the test. Get it to compile. Get it to pass. Tidy after if appropriate.

By the time we are done we should have a plug replacement for BTreeMap. In the rest of this document, fill in the names of tests that will encourage us to develop the code step by step. If during implementation you notice a test is needed that is not in the list, add it at the appropriate place in the list. As you complete tests, cross them off the list.

Each commit should have all the tests passing. Under no circumstances should you erase or alter tests just to get a commit to pass. If there is a genuine bug in a test, fix the test, but note that in the commit message.

## Test Plan for BPlusTreeMap Implementation

### Basic Structure and Operations

1. Test creating an empty BPlusTreeMap
2. Test inserting a single key-value pair
3. Test retrieving a value by key
4. Test overwriting an existing key's value
5. Test removing a key-value pair
6. Test checking if a key exists
7. Test getting the number of elements (len)
8. Test checking if the map is empty

### Core Collection Traits

9. Test creating a BPlusTreeMap from an iterator (FromIterator)
10. Test extending a BPlusTreeMap with elements from an iterator (Extend)
11. Test converting a BPlusTreeMap into an iterator (IntoIterator)
12. Test Debug formatting
13. Test cloning a BPlusTreeMap (Clone)
14. Test creating an empty BPlusTreeMap with Default

### Map-Specific Traits

15. Test indexing syntax with Index<K>

### Iterator Methods

16. Test iterating over key-value pairs (iter)
17. Test iterating with mutable references (iter_mut)
18. Test consuming iteration (into_iter)
19. Test iterating over keys only (keys)
20. Test iterating over values only (values)
21. Test iterating over mutable values (values_mut)

### Ordered Map Operations

22. Test getting the first key-value pair
23. Test getting the last key-value pair
24. Test range iteration over a subset of keys
25. Test mutable range iteration
26. Test getting entries for manipulation

### Advanced Operations

27. Test appending one map to another
28. Test clearing all elements
29. Test retaining elements based on a predicate
30. Test entry API for conditional insertion/modification

### Edge Cases

31. Test behavior with very large number of elements
32. Test with complex key types that implement Ord
33. Test proper memory management (no leaks)

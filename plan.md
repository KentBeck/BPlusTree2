We are implementing a BPlusTreeMap in Rust. We are writing the code like Kent Beck would write it. Before modifying code we consider whether tidying first would make the change easier. Commits will be separated into commits that change the behavior of the code and commits that only change the structure of the code. Write the code one test at a time. Write the test. Get it to compile. Get it to pass. Tidy after if appropriate.

By the time we are done we should have a plug replacement for BTreeMap. If during implementation you notice a test is needed that is not in the list, add it at the appropriate place in the list. As you complete tests, cross them off the list. Only implement enough code to make the test you just wrote pass, along with all the previous tests. If you find you have implemented too much, git revert --hard & try again.

Each commit should have all the tests passing. Under no circumstances should you erase or alter tests just to get a commit to pass. If there is a genuine bug in a test, fix the test, but note that in the commit message.

## Test Plan for BPlusTreeMap Implementation

### Basic Structure and Operations

1. ~~Test creating an empty BPlusTreeMap~~ ✓
2. ~~Test inserting a single key-value pair~~ ✓
3. ~~Test retrieving a value by key~~ ✓
4. ~~Test overwriting an existing key's value~~ ✓
5. ~~Test removing a key-value pair~~ ✓
6. ~~Test checking if a key exists~~ ✓
7. ~~Test getting the number of elements (len)~~ ✓
8. ~~Test checking if the map is empty~~ ✓

### Core Collection Traits

9. ~~Test creating a BPlusTreeMap from an iterator (FromIterator)~~ ✓
10. ~~Test extending a BPlusTreeMap with elements from an iterator (Extend)~~ ✓
11. ~~Test converting a BPlusTreeMap into an iterator (IntoIterator)~~ ✓
12. ~~Test Debug formatting~~ ✓
13. ~~Test cloning a BPlusTreeMap (Clone)~~ ✓
14. ~~Test creating an empty BPlusTreeMap with Default~~ ✓

### Map-Specific Traits

15. ~~Test indexing syntax with Index<K>~~ ✓

### Iterator Methods

16. ~~Test iterating over key-value pairs (iter)~~ ✓
17. ~~Test iterating with mutable references (iter_mut)~~ ✓
18. ~~Test consuming iteration (into_iter)~~ ✓
19. ~~Test iterating over keys only (keys)~~ ✓
20. ~~Test iterating over values only (values)~~ ✓
21. ~~Test iterating over mutable values (values_mut)~~ ✓

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

### B+ Tree Specific Operations

31. Test leaf node splitting when exceeding branching factor
32. Test branch node splitting when exceeding branching factor
33. ~~Test multi-level tree creation (depth > 2)~~ ✓
34. Test traversing a multi-level tree
35. Test inserting into a multi-level tree
36. Test removing from a multi-level tree
37. Test rebalancing after removal
38. Test merging nodes after removal when underfilled

### Code Structure Refactoring Tests

39. Test extracting common tree traversal logic
40. Test separating node operations into dedicated modules
41. Test implementing proper recursive handling of branch nodes in iterators

### Edge Cases

42. Test behavior with very large number of elements
43. Test with complex key types that implement Ord
44. Test proper memory management (no leaks)
45. Test with keys at the boundaries of their range (min/max integers)
46. Test with keys that have the same hash but are different (hash collision)
47. Test with empty strings as keys or values
48. Test with very long keys or values (e.g., large strings)
49. Test with non-ASCII characters in keys or values
50. Test with unbalanced trees (many insertions/deletions on one side)
51. Test with repeated insertions and deletions in the same location
52. Test with operations that cause multiple node splits or merges
53. Test with operations that cause root node changes
54. Test with operations near the branching factor boundary
55. Test with custom comparators for key ordering
56. Test with reverse ordering of keys
57. Test with concurrent access from multiple threads
58. Test with thread safety and synchronization
59. Test with serialization and deserialization of the tree
60. Test with persistence across program restarts
61. Test with error handling during operations
62. Test with zero-sized types as keys or values
63. Test with self-referential structures as values
64. Test with floating-point numbers as keys (precision issues)
65. Test with very similar keys that test comparison precision
66. Test with random access patterns vs sequential access patterns
67. Test with extremely uneven distribution of keys
68. Test with pathological insertion orders that cause worst-case behavior
69. Test with keys that trigger edge cases in the comparison function
70. Test with operations that hit every code path in node splitting/merging
71. Test with operations during iteration (modification during traversal)

### Missing Abstractions for Simplification

72. Implement a cursor/position abstraction for tracking location in the tree
73. ~~Implement a node visitor pattern for unified tree traversal~~ ✓
74. ~~Create a common iterator abstraction to simplify iterator implementations~~ ✓
75. ~~Extract node operations (splitting, merging) into separate abstractions~~ ✓
76. Develop a unified tree traversal abstraction with different traversal orders
77. ~~Create a key-value pair (Entry) abstraction for simplified operations~~ ✓
78. ✅ Implement a node balancing abstraction for insertion and removal operations
79. ~~Refactor to eliminate unsafe code through better abstractions~~ ✓
80. Implement a path abstraction for tracking ancestry during tree operations
81. Create a node buffer abstraction to simplify node splitting and merging

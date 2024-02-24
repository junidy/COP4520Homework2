# Programming Assignment 2

## Problem 1

Problem 1 involves coordination between threads using a single boolean. The challenge in this problem was two-fold: the conceptual solution to the problem was essentially an exercise in logic puzzling. Then came the actual implementation of the simulation. I took great pains to ensure the constraints of the problem were properly simulated, ensuring that the guests (the threads) have no means of communication whatsoever beyond the cupcake (the mutexed boolean). Here is the correspondance between the scenario outlined in the problem and my solution:

| Problem concept | Implementation |
| - | - |
| The Minotaur | main thread |
| "Leader" guest | first child thread |
| Counted guests | second through Nth child threads |
| The presence or absence of a cupcake | `Arc<Mutex<bool>>` |
| Guests waiting their turn to enter the labyrinth | `thread::park()` |
| Leader guest letting the Minotaur know that everyone has been through | `Arc<AtomicBool>` |



## Problem 2

The Minotaur presents the guests with three strategies for resolving contention in accessing a shared resource (the showroom). They are as follows:

### 1. Spinlock
This is the naive solution to the problem. It is one that requires minimal coordination between threads and thus introduces minimal overhead but has no way of guaranteeing fairness of access to the resource. This solution would be adequate for low contention situations
### 2. Mutex
Adding a mutex lock is the next level of complexity ...
- 
### 3. Queue


Overall, none of the three solutions is objectively superior to the others. Each solution strikes its own balance in the tradeoff between fairness, efficiency, and safety. The choice of solution depends on the requirements of the particular situation.

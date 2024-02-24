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
This is the naive solution to the problem. It is one that requires minimal coordination between threads and thus introduces minimal overhead but has no way of guaranteeing fairness of access to the resource. This solution would be adequate for low contention situations, where the average amount of waiting on any given access is very short.
### 2. Mutex
Adding a mutex lock provides a rudimentary form of coordination, allowing threads to sleep or move on after a failed attempt to acquire a lock instead of idling and burning processor cycles. Standard mutexes still have no way of assuring fairness, and introduces complexity that must be managed in order to preclude deadlocks.
### 3. Queue
A queueing based solution is the only one of the three that can guarantee fairness - it is inherent to the notion of FIFO. However, it introduces the most overhead, requiring the maintainence of an auxillary data structure, and its performance may degrade under high contention to a greater extent than the other two strategies, from contention over the queue itself.

Overall, none of the three solutions is objectively superior to the others. Each solution strikes its own balance in the tradeoff between fairness, efficiency, and safety. The choice of solution depends on the requirements of the particular situation.

I opted to implement strategy 2. I simulated the guests attempting to enter the showroom (acquiring the lock on the mutex) at occasional intervals. If the guest succeeds in entering the room, they will admire the vase (hold the lock) for another interval before exiting (unlocking). I specifically chose to have the guests `try_lock()` instead of merely `lock()`ing, so as to allow them to mingle at the party (perform other useful work) instead of sleeping until the room is unoccupied.
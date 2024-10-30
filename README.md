# algorithms

# Locks

| Lock                             | Time (ns) |
|----------------------------------|-----------|
| SpinLock lock                    | 18.817    |
| SpinLock lock and set            | 21.567  |
| Mutex lock                       | 20.851  |
| Mutex lock and set               | 23.328  |
| RwLock read                      | 22.439  |
| RwLock write                     | 22.697  |
| RwLock write and set             | 25.519  |
| parking_lot Mutex lock           | 22.652  |
| parking_lot Mutex lock and set   | 23.267  |
| parking_lot RwLock read          | 22.960  |
| parking_lot RwLock write         | 22.590  |
| parking_lot RwLock write and set | 25.198  |
    
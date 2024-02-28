#include <bits/stdc++.h>
using namespace std;
template <typename T>
inline T next() {
    T x;
    cin >> x;
    return x;
}
void solve() {
    int n = next<int>();
    int x = 0;
    for (int i = 0; i < n; i++) {
        x += next<int>();
    }
    int extra = (3 - x % 3) % 3;
    cout << (x + extra) / 3 << ' ' << extra << endl;
}
int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    solve();
    return 0;
}

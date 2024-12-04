#include<bits/stdc++.h>

using namespace std;

typedef long long ll;

#define pb push_back
#define sz(x) (int)(x.size())
#define itr(x) x.begin(), x.end()
#define ref(x) (*(x))
#ifdef LOC
#define debug(...) cerr << #__VA_ARGS__ << " : "; for(auto& dval : {__VA_ARGS__}) cerr << dval << " "; cerr << "\n";
#else 
#define debug(...)
#endif

mt19937 rng(chrono::steady_clock::now().time_since_epoch().count());

template<typename T> 
ostream& operator << (ostream& out, vector<T> v) {
    for(auto& i : v) {
        out << i << " ";
    }
    return out;
}

template<typename T, size_t SIZE> 
ostream& operator << (ostream& out, array<T,SIZE> v) {
    for(auto& i : v) {
        out << i << " ";
    }
    return out;
}

template<typename L, typename R>
ostream& operator << (ostream& out, pair<L,R> p) {
    out << "(" << p.first << ", " << p.second << ") ";
    return out;
}

// [ ] chill for one minute and evalute if the solution can be simpler

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0);

    vector<ll> a, b;
    string line;
    while (getline(std::cin, line)) {
        stringstream ss(line);
        ll v1, v2;
        ss >> v1 >> v2;
        a.push_back(v1);
        b.push_back(v2);
    }

    map<ll, ll> count_in_b;
    for(auto& i : b) {
        count_in_b[i]++;
    }
    ll sim_score = 0;
    for(auto& i : a) {
        debug(i, count_in_b[i]);
        sim_score += count_in_b[i] * i;
    }

    cout << "There were " << a.size() << " lines\n";
    cout << "The similarity score is " << sim_score << "\n";
}

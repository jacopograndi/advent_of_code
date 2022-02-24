#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <set>
#include <vector>
#include <algorithm>


struct Game {
	std::map<int, std::vector<int>> stars;
	std::set<int> single_cost;
	std::vector<int> cost;
	std::vector<int> goal;
};

std::vector<Game> games {
	{
		{
			{ 0, { 1 } },
			{ 1, { 0, 2, 7 } },
			{ 2, { 1, 7, 9, 3 } },
			{ 3, { 2, 9, 11, 4 } },
			{ 4, { 3, 11, 13, 5 } },
			{ 5, { 4, 13, 6 } },
			{ 6, { 5 } },
			{ 7, { 1, 2, 8 } },
			{ 8, { 7 } },
			{ 9, { 2, 3, 10 } },
			{ 10, { 9 } },
			{ 11, { 3, 4, 12 } },
			{ 12, { 11 } },
			{ 13, { 4, 5, 14 } },
			{ 14, { 13 } },
		},
		{ 0, 8, 10, 12, 14, 6 },
		{ 1, 10, 100, 1000 },
		{ 8, 10, 12, 14 }, 
	},
	{
		{
			{ 0, { 1 } },
			{ 1, { 0, 2, 7 } },
			{ 2, { 1, 7, 11, 3 } },
			{ 3, { 2, 15, 11, 4 } },
			{ 4, { 3, 15, 19, 5 } },
			{ 5, { 4, 19, 6 } },
			{ 6, { 5 } },
			{ 7, { 1, 2, 8 } },
			{ 8, { 7, 9 } },
			{ 9, { 8, 10 } },
			{ 10, { 9 } },		
			{ 11, { 2, 3, 12 } },
			{ 12, { 11, 13 } },
			{ 13, { 12, 14 } },
			{ 14, { 13 } },
			{ 15, { 3, 4, 16 } },
			{ 16, { 15, 17 } },
			{ 17, { 16, 18 } },
			{ 18, { 17 } },
			{ 19, { 4, 5, 20 } },
			{ 20, { 19, 21 } },
			{ 21, { 22, 20 } },
			{ 22, { 21 } },
		},
		{ 0, 8, 9, 10, 12, 13, 14, 16, 17, 18, 20, 21, 22, 6 },
		{ 1, 10, 100, 1000 },
		{ 10, 14, 18, 22 }, 
	}
};


class State {
	public:
	State () { }
	State (std::vector<int> pos): pos(pos) { }
	State (const State& o): pos(o.pos) { }

	std::vector<int> pos;
	
	bool operator==(const State &s) const { return pos==s.pos; }

	void dumb_parse (std::string raw) {
		auto newline = raw.find("\n");
		raw = raw.substr(newline+1);
		newline = raw.find("\n");
		raw = raw.substr(newline+1);
		
		std::string chars;
		chars += std::string{ raw[3] } 
			+ std::string{ raw[5] } 
			+ std::string{ raw[7] }
			+ std::string{ raw[9] };

		newline = raw.find("\n");
		raw = raw.substr(newline+1);
		
		chars += std::string{ raw[3] } 
			+ std::string{ raw[5] } 
			+ std::string{ raw[7] }
			+ std::string{ raw[9] };

		int a = 0, b = 0, c = 0, d = 0;
		for (int i=0; i<chars.size(); i++) pos.push_back(0);
		
		for (int i=0; i<chars.size(); i++) {
			int p = 0; 
			if (chars[i] == 'A') { p = 0+a; a++; }
			if (chars[i] == 'B') { p = 2+b; b++; }
			if (chars[i] == 'C') { p = 4+c; c++; }
			if (chars[i] == 'D') { p = 6+d; d++; }
			pos[p] = 7 + (i%4)*2+ (i/4);
		}
	}
	
	State enlarge() {
		State big;
		for (int i=0; i<16; i++) big.pos.push_back(0);
		std::vector<int> row0 { 20,16,12,8 };
		std::vector<int> row1 { 17,13,21,9 };
		for (int i=0; i<4; i++) {
			int a = (pos[i*2]-7) / 2;
			int b = 1-(pos[i*2])%2;
			big.pos[i*4] = pos[i*2]+a*2+b*2;
			a = (pos[i*2+1]-7) / 2;
			b = 1-(pos[i*2+1])%2;
			big.pos[i*4+3] = pos[i*2+1]+a*2+b*2;
			big.pos[i*4+1] = row0[i];
			big.pos[i*4+2] = row1[i];
		}
		return big;
	}

	std::string show () {
		std::string rep { "(State: [" };
		for (int i=0; i<pos.size(); i++) {
			rep += std::to_string(pos[i]);
			if (i<pos.size()-1) rep += ", ";
		}
		rep += "])";
		return rep;
	}

	bool done () {
		int size = pos.size() == 8 ? 0 : 1;
		for (int i=0; i<pos.size(); i++) {
			int type = i/(2+size*2);
			int goal = games[size].goal[type];
			if (size == 0) {
				if (pos[i] != goal && pos[i] != goal-1)
					return false;
			} else
			{
				if (pos[i] != goal && pos[i] != goal-1
						&& pos[i] != goal-2 && pos[i] != goal-3)
					return false;
			}
		}
		return true;
	}

	int mask () {
		int m = 0;
		for (int p : pos) m += 1 << p;
		return m;
	}
};


using Paths = std::vector<std::pair<int, int>>;
using AllPaths = std::map<int, std::map<int, std::map<int, Paths>>>;

int bitsum (int n) {
	int sum = 0;
	while (n > 0) {
		sum += n % 2;
		n /= 2;
	}
	return sum;
}

Paths get_paths (int mask, int pos, int type, int size=0) {
	std::vector<std::pair<int, int>> frontier { { pos, pos }  };
	std::map<int, int> visited;
	while (frontier.size() > 0) {
		std::pair<int, int> n = frontier[0];
		frontier.erase(frontier.begin());
		if (!visited.contains(n.first)) {
			visited.insert(n);
			for (int s : games[size].stars[n.first]) {
				if (((mask >> s) & 1) == 0) {
					frontier.push_back(std::pair<int, int> { s, n.first } );
				}
			}
		}
	}
	
	Paths paths;
	for (auto v : visited) {
		int energy = 0;
		int iter { v.first };
		while (iter != pos) {
			energy += 1;
			int prev = visited[iter];
			if (!games[size].single_cost.contains(iter) 
					&& !games[size].single_cost.contains(prev))
				energy ++;
			iter = visited[iter];
		}

		if (energy == 0) continue;

		int end = v.first;
		int goal = games[size].goal[type];
		if (end == goal) 
			return Paths { std::pair<int, int> { end, energy } } ;

		if (pos == goal) continue;
		if (pos < 7 && end < 7) continue;
		
		if (end == goal-1 
			&& (((mask >> goal) & 1) == 0)) continue;
		
		if (size == 0) {
			if (end >= 7 
					&& (end != goal-1 
					&& end != goal)) continue; 
		}
		else if (size == 1) {
			if (end == goal-2 
					&& (((mask >> (goal-1)) & 1) == 0)) continue;
			if (end == goal-3 
					&& (((mask >> (goal-2)) & 1) == 0)) continue;
			if (end >= 7 
					&& end != goal-3
					&& end != goal-2
					&& end != goal-1 
					&& end != goal) continue; 
		}
		
		paths.push_back(std::pair<int, int> { end, energy } );
	}
	return paths;
}

AllPaths construct_all_paths (int size=0) {
	AllPaths all;
	int i=0;
	int perms =   32768, pieces =  8, cells = 15;
	if (size == 1) {
		perms = 8388608; pieces = 16; cells = 23; 
	}

	for (int mask=0; mask<perms; mask++) {
		if (bitsum(mask) == pieces) {
			for (int pos=0; pos<cells; pos++) {
				for (int type=0; type<4; type++) {
					all[mask][pos][type] = 
						get_paths (mask, pos, type, size);
					i++;
				}
			}
		}
	}
	std::cout << "lookup size: " << i << std::endl;
	return all;
}

void next_states_amphipod (AllPaths &all, State state, int energy, int i, 
		std::multimap<int, State> &states, int size=0)
{
	std::multimap<int, State> next;
	int init = state.pos[i];
	int type = i/(2+size*2);
	for (auto p : all[state.mask()][init][type]) {
		int goal = games[size].goal[type];

		if (size == 0) {
			int other = (i%2==0 ? 1 : 0) + (i/2)*2;
			if (init == goal-1) 
				if (state.pos[other] == goal) continue;

			if (p.first == goal-1) {
				if (state.pos[other] == goal) {
					State n { state };
					n.pos[i] = p.first;

					int e = p.second * games[size].cost[type]+energy;
					bool found = false;
					auto range = states.equal_range(e);
					for (auto j = range.first; j != range.second; ++j) {
						if (j->second == n) { found = true; break; }
					}
					if (!found) states.insert({ e, n });
					return;
				} else continue;
			}
		} else {
			if (goal-3 <= init && init <= goal-1) {
				auto it = std::find(state.pos.begin(), state.pos.end(), 
					init+1);
				int other = it-state.pos.begin();
				if (other/4 == type) continue;
			}
			if (goal-3 <= p.first && p.first <= goal-1) {
				auto it = std::find(state.pos.begin(), state.pos.end(), 
					p.first+1);
				int other = it-state.pos.begin();
				if (other/4 == type) {
					State n { state };
					n.pos[i] = p.first;
					states.insert(
						{ p.second * games[size].cost[type]+energy, n }
					);
					return;
				} else continue;
			}
		}

		State n { state };
		n.pos[i] = p.first;
		int e = p.second * games[size].cost[type]+energy;
		bool found = false;
		auto range = states.equal_range(e);
		for (auto j = range.first; j != range.second; ++j) {
			if (j->second == n) { found = true; break; }
		}
		if (!found) states.insert({ e, n });
	}
	for (auto n : next) {
		states.insert(n);
	}
}

int search (AllPaths &all, State init) {
	std::vector<State> done;
	std::multimap<int, State> states { { 0, init } };
	
	
	int size = init.pos.size() == 8 ? 0 : 1;

	int j=0;
	int threshold = 0;
	while (states.size() > 0) {
		auto it = states.upper_bound(-1);
		int energy = (*it).first;		
		State s { (*it).second };		
		states.erase(it);

		if (j%10000 == 0)
			std::cout << states.size() << "\t" << energy << "\t" 
				<< s.show() << std::endl;
		j++;
	
		if (!s.done()) {
			for (int i=0; i<s.pos.size(); i++) {
				next_states_amphipod(all, s, energy, i, states, size);
			}
		} else { return energy; }

	}
	return 0;
}


// tests

bool test_paths_start () {
	State s { { 7,8,9,10,11,12,13,14 } };
	auto paths = get_paths(s.mask(), 7, 0); 
	std::cout << "res paths 1: " << paths.size() << std::endl;
	return paths.size() == 7;
}

bool test_paths_single () {
	State s { { 0,8,9,10,11,12,13,14 } };
	auto paths = get_paths(s.mask(), 0, 0); 
	std::cout << "res paths 2: " << paths.size() << std::endl;
	return paths.size() == 1;
}

bool test_paths_to_goal () {
	State s { { 9,10,0,1,11,12,13,14 } };
	auto paths = get_paths(s.mask(), 9, 0);
	std::cout << "res paths 3: " << paths.size() << std::endl;
	for (auto p : paths) {
		std::cout << "  " << p.first << " " <<p.second << std::endl;
	}
	return paths.size() == 1;
}

bool test_paths_to_goal_from_below () {
	State s { { 8,10,0,1,11,12,13,14 } };
	auto paths = get_paths(s.mask(), 10, 0);
	std::cout << "res paths 4: " << paths.size() << std::endl;
	for (auto p : paths) {
		std::cout << "  " << p.first << " " <<p.second << std::endl;
	}
	return paths.size() == 6;
}

bool test_paths_stuck () {
	State s { { 8,14,10,9,11,12,13,6 } };
	auto paths = get_paths(s.mask(), 13, 0);
	std::cout << "res paths 5: " << paths.size() << std::endl;
	for (auto p : paths) {
		std::cout << "  " << p.first << " " <<p.second << std::endl;
	}
	return paths.size() == 7;
}

bool test_search_one_step (AllPaths all) {
	auto s = search(all, State { { 0,8,9,10,11,12,13,14 } });
	std::cout << "res search 1: " << s << std::endl;
	return s != 0;
}

bool test_search_two_step (AllPaths all) {
	auto s = search(all, State { { 0,8,1,10,11,12,13,14 } });
	std::cout << "res search 2: " << s << std::endl;
	return s != 0;
}

bool test_search_from_goal (AllPaths all) {
	auto s = search(all, State { { 9,10,0,1,11,12,13,14 } });
	std::cout << "res search 3: " << s << std::endl;
	return s != 0;
}

bool test_search_stuck (AllPaths all) {
	auto s = search(all, State { { 8,14,10,9,11,12,13,6 } });
	std::cout << "res search 4: " << s << std::endl;
	return s != 0;
}

bool test_search_strange (AllPaths all) {
	auto s = search(all, State { { 13,8,9,10,11,12,7,14 } });
	std::cout << "res search 5: " << s << std::endl;
	return s != 0;
}

bool test_paths_big () {
	State s { 
		{ 10,17,20,22,7,13,15,16,11,12,18,21,8,9,14,19 } 
	};
	auto paths = get_paths(s.mask(), 7, 0, 1);
	std::cout << "res big paths 1: " << paths.size() << std::endl;
	for (auto p : paths) {
		std::cout << "  " << p.first << " " <<p.second << std::endl;
	}
	return paths.size() == 7;
}

bool test_paths_big_simple () {
	State s { 
		{  7, 8, 9,10,11,12,13,14,15,16,17,18, 0 ,20,21,22 } 
	};
	auto paths = get_paths(s.mask(), 0, 3, 1);
	std::cout << "res big paths 2: " << paths.size() << std::endl;
	for (auto p : paths) {
		std::cout << "  " << p.first << " " <<p.second << std::endl;
	}
	return paths.size() == 1;
}

bool test_big_done () {
	State s { 
		{  7, 8, 9,10,11,12,13,14,15,16,17,18,19,20,21,22 } 
	};
	State n { 
		{  7, 8, 9,10,11,12,13,14,15,16,17,18,0,20,21,22 } 
	};
	std::cout << "res big done: " << s.done() << std::endl;
	std::cout << "res big not done: " << n.done() << std::endl;
	return s.done() && !n.done();
}

bool test_search_big_simple (AllPaths big) {
	auto s = search(big, State { 
		{  7, 8, 9,10,11,12,13,14,15,16,17,18, 0 ,20,21,22 } 
	});
	std::cout << "res big search 1: " << s << std::endl;
	return s != 0;
}

bool test_search_big (AllPaths big) {
	auto s = search(big, State { 
		{ 10,17,20,22,7,13,15,16,11,12,18,21,8,9,14,19 } 
	});
	std::cout << "res big search 2: " << s << std::endl;
	return s != 0;
}

void tests (int size) {
	int succ = 0, fail = 0;
	
	AllPaths all = construct_all_paths(); 
	if( test_paths_start () ) succ++; else fail++;
	if( test_paths_single () ) succ++; else fail++;
	if( test_paths_to_goal () ) succ++; else fail++;
	if( test_paths_to_goal_from_below () ) succ++; else fail++;
	if( test_paths_stuck () ) succ++; else fail++;
	if( test_search_one_step (all) ) succ++; else fail++;
	if( test_search_two_step (all) ) succ++; else fail++;
	if( test_search_from_goal (all) ) succ++; else fail++;
	if( test_search_stuck (all) ) succ++; else fail++;
	if( test_search_strange (all) ) succ++; else fail++;
	
	if (size == 1) {
		if( test_paths_big () ) succ++; else fail++;
		if( test_paths_big_simple () ) succ++; else fail++;
		if( test_big_done () ) succ++; else fail++;
		AllPaths big = construct_all_paths(1); 
		if( test_search_big_simple (big) ) succ++; else fail++;
		if( test_search_big (big) ) succ++; else fail++;
	}

	std::cout << "tests: " << succ<<"/"<<succ+fail << std::endl;
}


int main (int argc, char *argv[]) {
	if (std::string { argv[1] } == "-T") {
		tests(1);
		return 0;
	}
	if (std::string { argv[1] } == "-t") {
		tests(0);
		return 0;
	}
	
	AllPaths all = construct_all_paths(); 
	AllPaths big = construct_all_paths(1); 

	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	State state; state.dumb_parse(raw);
	
	std::cout << state.show() << std::endl;
	int cost = search(all, state);
	std::cout << "cost: " << cost << std::endl;

	std::cout << state.enlarge().show() << std::endl;
	cost = search(big, state.enlarge());
	std::cout << "cost big: " << cost << std::endl;
	return 0;
}

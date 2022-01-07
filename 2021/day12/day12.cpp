#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <stack>
#include "../utils.h"

class Arc {
	public: Arc (std::string s, std::string t) : s(s), t(t) {}
	std::string s, t;
};

class Graph {
	public: Graph () { }
	std::vector<std::string> nodes;
	std::vector<Arc> arcs;

	std::vector<std::string> star (std::string start) {
		std::vector<std::string> res;
		for (auto arc : arcs) {
			if (arc.s == start) res.push_back(arc.t);
			if (arc.t == start) res.push_back(arc.s);
		}
		return res;
	}
};

int count (std::vector<std::string> vec, std::string str) {
	return std::count(std::begin(vec), std::end(vec), str);
}

bool lower (std::string str) {
	for (auto c : str) if (!islower(c)) return false;
	return true;
}

bool check (std::vector<std::string> vec, std::string str, bool twice) {
	if (!lower(str)) return true;
	if (count(vec, str) < 1) return true;
	if (twice) {
		std::string flag = "";
		for (auto s : vec) {
			if (count(vec, s) > 1 && lower(s)) {
				if (flag == "") flag = s;
				else return false;
			}
		}
		return true;
	}
	return false;
} 

using Paths = std::vector<std::vector<std::string>>;
Paths follow (Graph G, std::vector<std::string> path, bool twice) {
	auto c = path[path.size()-1];
	auto star = G.star(c);
	Paths paths;
	if (c == "end") {
		paths.push_back(path);
		return paths;
	}
	for (auto n : star) {
		if (n == "start") continue;
		if (!check(path, n, twice)) continue;

		std::vector<std::string> newpath = path;
		newpath.push_back(n);
		Paths res = follow(G, newpath, twice);
		paths.insert(std::end(paths), std::begin(res), std::end(res));
	}
	return paths;
}

Paths search (Graph G, bool twice) {
	return follow(G, std::vector<std::string> { "start" }, twice);
}


int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> lines;
	split(lines, raw, "\n");
	
	Graph graph;
	for (auto line : lines) {
		if (line.size() == 0) continue; 
		std::vector<std::string> tokens;
		split(tokens, line, "-");
		graph.nodes.push_back(tokens[0]);
		graph.nodes.push_back(tokens[1]);
		graph.arcs.push_back(Arc { tokens[0], tokens[1] });
	}
	
	auto paths = search(graph, (argc > 2 ? (atoi(argv[2]) == 1) : false));
	for (auto p : paths) {
		for (auto s : p) {
			std::cout << s << ",";
		}
		std::cout << std::endl;
	}

	std::cout << "paths: " << paths.size() << std::endl;

	return 0;
}

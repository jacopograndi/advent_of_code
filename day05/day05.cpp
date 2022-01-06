#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <map>

#include "../utils.h"

class Line {
	public:
	Line (int a, int b, int c, int d) : a(a), b(b), c(c), d(d) { }
	Line (std::string line) {
		std::vector<std::string> parts, start, end;
		split(parts, line, "->");
		split(start, parts[0], ",");
		split(end, parts[1], ",");
		a = std::stoi(start[0]);
		b = std::stoi(start[1]);
		c = std::stoi(end[0]);
		d = std::stoi(end[1]);
		if (c-a == 0) xdir = 0;
		else { xdir = (c-a) > 0 ? 1 : -1; }
		if (d-b == 0) ydir = 0;
		else { ydir = (d-b) > 0 ? 1 : -1; }
	}
	int a, b, c, d, xdir, ydir;

	int length () { return abs(a-c) + abs(b-d); }

	bool operator==(Line& oth) {
		return a == oth.a && b == oth.b && c == oth.c && d == oth.d;
	}
};

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');

	std::vector<Line> lines;
	std::vector<std::string> strlines;
	split(strlines, raw, "\n");
	for (auto strline : strlines) {
		if (strline.size() == 0) continue;
		Line line { strline };
		if (line.xdir != 0 && line.ydir != 0 
				&& std::stoi(argv[2]) == 0) continue;
		lines.push_back(line);
	}

	int intersections = 0;
	std::map<std::pair<int, int>, int> overlaps;
	for (auto& line : lines) {
		int dist = 0;
		bool straight = true;
		if (line.xdir != 0 && line.ydir != 0) straight = false;
		for (int i=0; dist<line.length()+1; i++) {
			std::pair pair { line.a + line.xdir*i, line.b + line.ydir*i };
			if (overlaps.count(pair) == 0) overlaps[pair] = 1;
			else {
				if (overlaps[pair] == 1) intersections += 1;
				overlaps[pair] += 1;
			}
			if (straight) dist ++;
			else dist += 2;
		}
	}


	int minx = 200;
	int maxx = -200;
	int miny = 200;
	int maxy = -200;
	for (auto& line : lines) {
		minx = std::min(minx, std::min(line.a, line.c));
		maxx = std::max(maxx, std::max(line.a, line.c));
		miny = std::min(miny, std::min(line.b, line.d));
		maxy = std::max(maxy, std::max(line.b, line.d));
	}

	std::cout << "bounds: " 
		<< minx << " " << maxx << " "
		<< miny << " " << maxx << std::endl;

	/* visualizer
	for (int y=miny; y<maxy+1; y++) {
		for (int x=minx; x<maxx+1; x++) {
			int amt = overlaps[std::make_pair(x, y)];
			if (amt > 9) amt = 9;
			if (amt > 0) std::cout << amt;
			else std::cout << ".";

		}
		std::cout << std::endl;
	}
	*/

	std::cout << "intersections: " << intersections << std::endl;

	return 0;
}

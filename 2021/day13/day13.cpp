#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include "../utils.h"

class Fold {
	public: Fold(bool axis, int i) : axis(axis), i(i) { }
	int i; bool axis;
};

class Dot {
	public: Dot(int x, int y) : x(x), y(y) { }
	int x, y;
};


void show (std::vector<Dot> dots, bool vis) {
	int minx = 99999, maxx = -99999;
	int miny = 99999, maxy = -99999;
	for (Dot dot : dots) {
		minx = std::min(minx, dot.x);
		miny = std::min(miny, dot.y);
		maxx = std::max(maxx, dot.x);
		maxy = std::max(maxy, dot.y);
	}
	int count = 0;
	int sizex = maxx-minx+1;
	int sizey = maxy-miny+1;
	for (int y=0; y<sizey; y++) {
		for (int x=0; x<sizex; x++) {
			bool occ = false;
			for (Dot dot : dots) {
				if (dot.x == x && dot.y == y) {
					occ = true; break;
				}
			}
			if (occ) {
				count ++;
				if (vis) std::cout << "#";
			}
			else if (vis) std::cout << ".";
		}
		if (vis) std::cout << std::endl;
	}

	std::cout << "dots remaining: " << count << std::endl;
}

void fold (std::vector<Dot> &dots, Fold f) {
	for (Dot &dot : dots) {
		if (f.axis) { 
			int dist = f.i - dot.x;
			if (dist < 0) dot.x = f.i + dist; 
		}
		else {
			int dist = f.i - dot.y;
			if (dist < 0) dot.y = f.i + dist; 
		}
	}
}

int main (int argc, char * argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<std::string> parts, strdots, strfolds;
	split(parts, raw, "\n\n");
	split(strdots, parts[0], "\n");
	split(strfolds, parts[1], "\n");

	std::vector<Dot> dots;
	for (auto strdot : strdots) {
		std::vector<std::string> xy;
		split(xy, strdot, ",");
		dots.emplace_back(
			std::stoi(xy[0]),
			std::stoi(xy[1]));
	}

	std::vector<Fold> folds;
	for (auto strfold : strfolds) {
		if (strfold.size() == 0) continue;
		std::vector<std::string> com;
		split(com, strfold, "=");
		folds.emplace_back(
			com[0] == "fold along x", 
			std::stoi(com[1]));
	}

	int iter = folds.size(); 
	if (argc > 2) iter = atoi(argv[2]);
	for (Fold f : folds) {
		fold(dots, f);
	}
	show(dots, true);


	return 0;
}

#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <vector>

using vec = std::pair<int, int>;
using lattice = std::map<vec, int>;


std::vector<int> get_bounds (lattice img) {
	std::vector<int> bounds { 999999, -999999, 999999, -999999 };
	for (auto k : img) {
		bounds[0] = std::min(bounds[0], k.first.first);
		bounds[1] = std::max(bounds[1], k.first.first);
		bounds[2] = std::min(bounds[2], k.first.second);
		bounds[3] = std::max(bounds[3], k.first.second);
	}
	return bounds;
}

void show_lattice (lattice img) {
	auto bounds = get_bounds(img);
	std::cout << "lattice [" << 
		bounds[0] <<", " << bounds[1] << ", " << 
		bounds[2] <<", " << bounds[3] << "]" << std::endl;
	for (int y=bounds[2]; y<=bounds[3]; y++) {
		for (int x=bounds[0]; x<=bounds[1]; x++) {
			if (img[vec(x,y)] == 1) 
				std::cout << "#";
			else std::cout << ".";
		}
		std::cout << std::endl;
	}
}

lattice parse_lattice (std::string raw) {
	lattice img;
	int y = 0;
	while (raw.size() > 0) {
		auto newline = raw.find("\n");
		std::string line = raw;
		if (newline != std::string::npos) {
			line = raw.substr(0, newline);
			raw = raw.substr(newline+1);
		}
		for (int x=0; x<line.size(); x++) {
			if (line[x] == '#') img[vec(x, y)] = 1;
			else img[vec(x, y)] = 0;
		}
		y ++;
	}
	return img;	
}


class Matrix {
	public:
	Matrix () { background = 0; }
	Matrix (lattice img) {
		auto bounds = get_bounds(img);
		sx = bounds[1]+1 - bounds[0];
		sy = bounds[3]+1 - bounds[2];
		offx = bounds[0];
		offy = bounds[2];
		for (int y=bounds[2]; y<=bounds[3]; y++) {
			for (int x=bounds[0]; x<=bounds[1]; x++) {
				cells.push_back(img[vec(x,y)]);
			}
		}
	}

	lattice to_lattice () {
		lattice img;
		for (int y=0; y<sy; y++) {
			for (int x=0; x<sx; x++) {
				img[vec(x+offx,y+offy)] = cells[at(x,y)];
			}
		}
		return img;
	}

	int at (int x, int y) { return x + y*sx; }
	int get_check (int x, int y) {
		if (x<0 || x>=sx || y<0 || y>=sy) return background;	
		return cells[at(x, y)]; 
	}

	std::vector<int> cells;
	int sx, sy;
	int offx, offy;
	int background;

	Matrix trim (int shrink) {
		Matrix trimmed;
		trimmed.offx = offx + shrink;
		trimmed.offy = offy + shrink;
		trimmed.sx = sx - shrink*2;
		trimmed.sy = sy - shrink*2;
		trimmed.background = background;
		for (int y=0; y<trimmed.sy; y++) {
			for (int x=0; x<trimmed.sx; x++) {
				trimmed.cells.push_back(cells[at(x+shrink, y+shrink)]);
			}
		}
		return trimmed;
	}
};


int get_rule_index (Matrix m, int s, int t) {
	int index = 0;
	for (int j=-1; j<2; j++) {
		for (int i=-1; i<2; i++) {
			int val = m.get_check(s+i,t+j);
			int shift = 8 - ((i+1)+(j+1)*3);
			index += (val << shift);
		}
	}
	return index;
}

Matrix apply_rules (std::string rules, Matrix prev) {
	Matrix next;
	int expand = 3;
	next.sx = prev.sx + expand*2;
	next.sy = prev.sy + expand*2;
	next.offx = prev.offx - expand;
	next.offy = prev.offy - expand;
	if (prev.background == 0) {
		next.background = rules[0] == '#' ? 1 : 0;
	}
	if (prev.background == 1) {
		next.background = rules[511] == '#' ? 1 : 0;
	}
	for (int y = 0; y < next.sy; y++) {
		for (int x = 0; x < next.sx; x++) {
			int index = get_rule_index(prev, x-expand, y-expand);
			int val = (rules[index] == '#') ? 1 : 0;
			next.cells.push_back(val);
		}
	}
	Matrix trimmed = next.trim(expand-1);
	return next;
}

int lit (std::vector<int> bounds, lattice img) {
	int sum = 0;
	for (int y = bounds[2]; y <= bounds[3]; y++) {
		for (int x = bounds[0]; x <= bounds[1]; x++) {
			sum += img[vec(x,y)];
		}
	}
	return sum;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	auto separator = raw.find("\n\n");
	std::string rules = raw.substr(0, separator);
	std::string strimg = raw.substr(separator+2);
	lattice img = parse_lattice(strimg);
	show_lattice(img);
	auto bounds = get_bounds(img);
	int maxiter = std::stoi(std::string { argv[2] } );

	Matrix m { img };
	for (int i=0; i<maxiter; i++) {
		m = apply_rules(rules, m);
		std::cout << "iter: " << i << std::endl;
	}
	lattice out = m.to_lattice();
	
	show_lattice(out);
	bounds[0] -= maxiter;
	bounds[1] += maxiter;
	bounds[2] -= maxiter;
	bounds[3] += maxiter;
	std::cout << "lit " << lit(bounds, out) << std::endl;
	
	return 0;
}

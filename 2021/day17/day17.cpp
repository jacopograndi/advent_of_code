#include <iostream>
#include <string>
#include <vector>
#include <map>

// -- notes
// if vy(0) > 0 the trajectory always reaches y=0 at time t 
//     and vy(t) = -vy(0), vy(t+1) = -vy(0)-1
// sum {i:1..n} (n) = ((n)(n+1))/2
// triangle = { i in N | exist an n in N s.t. i = sum(n) }
// if probe reaches y=0 with vx(t)=0, vx(0) is in triangle
// so for every vx(0) in sumn and sum(vx(0)) in target
//     get the largest vy(t) in target
// this approach only works if vy(0)>0

std::vector<int> parse_range(std::string raw) {
	std::vector<int> range;
	std::string nolabel = raw.substr(2);
	auto dotdot = nolabel.find("..");
	std::string r = nolabel.substr(0, dotdot);
	std::string l = nolabel.substr(dotdot+2);
	range.push_back(std::stoi(r));
	range.push_back(std::stoi(l));
	return range; 
}

int get_trig (int n) {
	return ((n)*(n+1))/2;
}

int validate (int vx, int vy, std::vector<int> range, int maxt) {
	int x = 0, y = 0;
	for (int t=0; t<maxt; t++) {
		x += vx; y += vy;
		if (vx > 0) vx--; 
		vy--;
		if (range[0] <= x && x <= range[1])
			if (range[2] <= y && y <= range[3])
				return 1;
		if (y < range[2]) return 0;
	}
	return 0;
}

int main (int argc, char *argv[]) {
	std::string raw = std::string({ argv[1] });
	std::string nolabel = raw.substr(13);
	auto comma = nolabel.find(", ");
	std::string rangex = nolabel.substr(0, comma);
	std::string rangey = nolabel.substr(comma+2);
	auto range = parse_range(rangex);
	auto ry = parse_range(rangey);
	range.insert(std::end(range), std::begin(ry), std::end(ry));
	// range is x1, x2, y1, y2
	
	std::map<int, int> triangles {};
	for (int i=0; i<100; i++) {
		triangles[get_trig(i)] = i;
	}

	int vx = 0;
	int vy = 0;
	for (int i=range[0]; i<range[1]; i++) {
		if (triangles.contains(i)) {
			for (int j=range[2]; j<range[3]; j++) {
				if (-j > vy) {
					vy = -j-1;
					vx = triangles[i];
				}
			}
		}
	}

	std::cout << "velocity: vx=" << vx << ", vy=" << vy << std::endl;
	std::cout << "max height: " << get_trig(vy) << std::endl;

	int sum = 0;
	for (int i=0; i<range[1]*2; i++) {
		for (int j=range[2]; j<vy+1; j++) {
			sum += validate(i, j, range, 1000);
		}
	}

	std::cout << "total intial velocities: " << sum << std::endl;

	return 0;
}

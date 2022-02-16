#include <iostream>
#include <fstream>
#include <string>
#include <vector>


class Beacon {
	public:
	Beacon() { x=y=z=0; }
	Beacon(int x, int y, int z): x(x), y(y), z(z) { }
	Beacon(const Beacon &b) { x=b.x; y=b.y; z=b.z; }

	int x,y,z;

	bool operator==(const Beacon& r) { 
		return x==r.x && y==r.y && z==r.z;
	}
	friend bool operator==(const Beacon& l, const Beacon& r) { 
		return l.x==r.x && l.y==r.y && l.z==r.z;
	}

	std::string show () {
		std::string rep = 
			std::to_string(x) + "," +
			std::to_string(y) + "," +
			std::to_string(z) + "\n";
		return rep;
	}
};

class Scanner {
	public:
	Scanner() { }
	Scanner(const Scanner &s) { 
		for (Beacon b : s.beacons) beacons.emplace_back(b);
	}

	int x,y,z;
	std::vector<Beacon> beacons;
	bool fixed = false;

	bool operator==(const Scanner& r) { 
		return x==r.x && y==r.y && z==r.z && beacons == r.beacons;
	}

	std::string show () {
		std::string rep = "scanner\n";
		for (Beacon b : beacons) {
			rep += "  beacon at " + b.show();
		}
		return rep;
	}

	Scanner scale (int x, int y, int z) {
		Scanner s;
		for (Beacon& b : beacons) {
			s.beacons.emplace_back(b.x*x, b.y*y, b.z*z);
		}
		return s;
	}

	Scanner rotate (int axis, int pi_halves) {
		Scanner s;
		int sin = pi_halves < 2 ? pi_halves : -pi_halves+2;
		pi_halves = (pi_halves+1) % 4;
		int cos = pi_halves < 2 ? pi_halves : -pi_halves+2;
		for (Beacon& b : beacons) {
			if (axis == 2)
				s.beacons.emplace_back(
					b.x*cos - b.y*sin, 
					b.x*sin + b.y*cos,
					b.z);
			if (axis == 1)
				s.beacons.emplace_back(
					b.x*cos + b.z*sin, 
					b.y,
					-b.x*sin + b.z*cos);
			if (axis == 0)
				s.beacons.emplace_back(
					b.x, 
					b.y*cos - b.z*sin,
					b.y*sin + b.z*cos);
		}
		return s;
	}

	Scanner rebase (bool swapxy, bool swapyz, bool swapxz) {
		Scanner s;
		int tmp;
		for (Beacon& b : beacons) {
			Beacon a { b };
			if (swapxy) { tmp = a.x; a.x = a.y; a.y = tmp; }
			if (swapyz) { tmp = a.y; a.y = a.z; a.z = tmp; }
			if (swapxz) { tmp = a.x; a.x = a.z; a.z = tmp; }
			s.beacons.push_back(a);
		}
		return s;
	}

	Scanner translate (int x, int y, int z) {
		Scanner s;
		for (Beacon& b : beacons) {
			s.beacons.emplace_back(b.x+x, b.y+y, b.z+z);
		}
		return s;
	}

	int overlap (Scanner oth) {
		int sum = 0;
		for (Beacon &b : beacons) {
			if (std::find(std::begin(oth.beacons), std::end(oth.beacons), b)
					!= std::end(oth.beacons)) {
				sum++;
			}
		}
		//std::cout << sum << " ";
		return sum;
	}
};


std::vector<Scanner> orientations (Scanner source) {
	std::vector<Scanner> orientations;
	for (int i=0; i<4; i++) {
		Scanner rotated = source.rotate(0, i);
		for (int j=0; j<6; j++) {
			Scanner based { rotated };
			if (j==1) {
				based = rotated.rotate(1, 1);
			}
			if (j==2) {
				based = rotated.rotate(1, 2);
			}
			if (j==3) {
				based = rotated.rotate(1, 3);
			}
			if (j==4) {
				based = rotated.rotate(2, 1);
			}
			if (j==5) {
				based = rotated.rotate(2, 3);
			}
			orientations.push_back(based);
		}
	}
	return orientations;
}

Beacon parse_beacon (std::string raw) {
	Beacon b;

	auto comma = raw.find(",");
	b.x = std::stoi(raw.substr(0, comma));
	raw = raw.substr(comma+1);

	comma = raw.find(",");
	b.y = std::stoi(raw.substr(0, comma));
	raw = raw.substr(comma+1);

	b.z = std::stoi(raw);
	
	return b;
}

std::vector<Scanner> parse_scanners (std::string raw) {
	std::vector<Scanner> scanners;
	while (raw.size() > 0) {
		auto newline = raw.find('\n');
		if (newline != std::string::npos) {
			std::string line = raw.substr(0, newline);
			raw = raw.substr(newline+1);
			if (line.find("---") != std::string::npos) {
				scanners.push_back(Scanner());
			} else if (line.size() > 1) {
				Beacon b = parse_beacon(line);
				scanners[scanners.size()-1].beacons.push_back(b);
			}
		} else {
			Beacon b = parse_beacon(raw);
			scanners[scanners.size()-1].beacons.push_back(b);
		}
	}
	return scanners;
}

int main (int argc, char *argv[]) {
	std::string raw; std::getline(std::ifstream(argv[1]), raw, '\0');
	
	std::vector<Scanner> scanners = parse_scanners(raw);

	std::vector<Beacon> cloud;
	for (Beacon b : scanners[0].beacons) {
		if (std::find(std::begin(cloud), std::end(cloud), b) 
				== std::end(cloud))
			cloud.push_back(b);
	}
	scanners[0].fixed = true;
	scanners[0].x = 0;
	scanners[0].y = 0;
	scanners[0].z = 0;

	while (true) {
		int fixed_count = 0;
		bool overlap = false;
		for (int i=0; i<scanners.size(); i++) {
			if (scanners[i].fixed) { 
				fixed_count ++;
				continue; 
			}
			for (int j=0; j<scanners.size(); j++) {
				if (i == j) continue;
				if (!scanners[j].fixed) continue;
				Scanner base = scanners[j];
				std::cout << "compare " << i << " against " << j << std::endl;
				auto rebaseds = orientations(scanners[i]);
				for (Scanner rebased : rebaseds) {
					for (Beacon as : base.beacons) {
						for (Beacon bas : rebased.beacons) {
							int dx = as.x - bas.x;
							int dy = as.y - bas.y;
							int dz = as.z - bas.z;
							int dist = std::abs(scanners[j].x-dx) 
								+ std::abs(scanners[j].y-dy)
							   	+ std::abs(scanners[j].z-dz);
							if (dist > 2000) continue;
							Scanner translated = rebased.translate(dx, dy, dz);
							if (base.overlap(translated) >= 12) {
								std::cout << "overlaps: " << base.overlap(translated) << std::endl;
								scanners[i].x = dx;
								scanners[i].y = dy;
								scanners[i].z = dz;
								std::cout << "  at: " <<dx<<" "<<dy<<" "<<dz << std::endl;
								std::cout << "  dist: " <<dist<< std::endl;
								scanners[i].beacons.clear();
								for (Beacon b : translated.beacons) {
									scanners[i].beacons.emplace_back(b);
									if (std::find(std::begin(cloud), std::end(cloud), b) 
											== std::end(cloud))
										cloud.push_back(b);
								}
								scanners[i].fixed = true;
								overlap = true;
								break;
							}
						}
						if (overlap) break;
					}
					if (overlap) break;
				}
				if (overlap) break;
			}
			if (overlap) break;
		}
		if (fixed_count == scanners.size()) break;
	}
	std::cout << "beacons: " << cloud.size() << std::endl;
	int maxdist = 0;
	for (int i=0; i<scanners.size(); i++) {
		for (int j=0; j<scanners.size(); j++) {
			if (i==j) continue;
			int dist = std::abs(scanners[i].x-scanners[j].x) + 
				std::abs(scanners[i].y-scanners[j].y) + 
				std::abs(scanners[i].z-scanners[j].z);
			maxdist = std::max(dist, maxdist);
		}
	}
	std::cout << "max dist: " << maxdist << std::endl;
	Scanner s;
	for (Beacon b : cloud) { s.beacons.push_back(b); }
	//std::cout << s.show() << std::endl;
	return 0;
}

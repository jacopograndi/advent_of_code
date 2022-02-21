#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <unordered_map>


const bool DEBUG = true;


class Vec {
	public: 
	Vec (): x(0), y(0), z(0) { }
	Vec (int x, int y, int z): x(x), y(y), z(z) { }
	Vec (const Vec &o): x(o.x), y(o.y), z(o.z) { }
	long x, y, z;

	bool operator==(const Vec& o) const { 
		return x==o.x && y==o.y && z==o.z;
   	}

	std::string show () { 
		return "[" + std::to_string(x) + ", "
			+ std::to_string(y) + ", "
			+ std::to_string(z) + "]";
	}
};


class Cuboid {
	public:
	Cuboid (): a(), b() { }
	Cuboid (Vec a, Vec b): a(a), b(b) { }
	Vec a, b;

	bool operator==(const Cuboid& o) const { 
		return a==o.a && b==o.b;
   	}

	std::string show () { 
		return "(Cuboid " + a.show() + ", " + b.show() + ")";
	}

	long volume () { return (b.x-a.x+1) * (b.y-a.y+1) * (b.z-a.z+1); }

	bool intersect(const Cuboid &o) const {
		if (a.x <= o.b.x && b.x >= o.a.x)
		if (a.y <= o.b.y && b.y >= o.a.y)
		if (a.z <= o.b.z && b.z >= o.a.z)
			return true;
		return false;
	}

	std::vector<Cuboid> cut (int axis, int h) {
		std::vector<Cuboid> pieces;
		if (axis == 0) {
			if (a.x < h && h <= b.x) {
				pieces.emplace_back(a, Vec(h-1, b.y, b.z));
				pieces.emplace_back(Vec(h, a.y, a.z), b);
			} else pieces.emplace_back(a, b);
		}
		if (axis == 1) {
			if (a.y < h && h <= b.y) {
				pieces.emplace_back(a, Vec(b.x, h-1, b.z));
				pieces.emplace_back(Vec(a.x, h, a.z), b);
			} else pieces.emplace_back(a, b);
		}
		if (axis == 2) {
			if (a.z < h && h <= b.z) {
				pieces.emplace_back(a, Vec(b.x, b.y, h-1));
				pieces.emplace_back(Vec(a.x, a.y, h), b);
			} else pieces.emplace_back(a, b);
		}
		return pieces;
	}

	int double_distance (Cuboid o) {
		return std::abs(a.x+b.x - o.a.x-o.b.x) + 
			std::abs(a.y+b.y - o.a.y-o.b.y) + 
			std::abs(a.z+b.z - o.a.z-o.b.z); 
	}

	Cuboid slice_step (std::vector<Cuboid> &slices, 
			Cuboid remainder, Cuboid other, int axis, int h) 
	{
		auto c = remainder.cut (axis, h);
		if (c.size() > 1) {
			if (other.double_distance(c[0]) > other.double_distance(c[1])) {
				slices.push_back(c[0]);
				remainder = c[1];
			} else { 
				slices.push_back(c[1]);
				remainder = c[0];
			}
		} 
		return remainder;
	}

	std::vector<Cuboid> slice_vec (
			std::vector<Cuboid> prev, int axis, int h) 
	{
		std::vector<Cuboid> next;
		for (Cuboid s : prev) {
			auto res = s.cut(axis, h);
			for (Cuboid c : res) next.push_back(c);
		}
		return next;
	}

	std::vector<Cuboid> slice (Cuboid o) {	
		
		std::vector<Cuboid> slices { Cuboid(a, b) };
		slices = slice_vec(slices, 0, o.a.x);
		slices = slice_vec(slices, 0, o.b.x+1);
		slices = slice_vec(slices, 1, o.a.y);
		slices = slice_vec(slices, 1, o.b.y+1);
		slices = slice_vec(slices, 2, o.a.z);
		slices = slice_vec(slices, 2, o.b.z+1);
		
		/* alternative implemetation, faster but creates overlaps
		std::vector<Cuboid> slices;
		Cuboid remainder { a, b };	
		remainder = slice_step(slices, remainder, o, 0, o.a.x);
		remainder = slice_step(slices, remainder, o, 0, o.b.x+1);
		remainder = slice_step(slices, remainder, o, 1, o.a.y);
		remainder = slice_step(slices, remainder, o, 1, o.b.y+1);
		remainder = slice_step(slices, remainder, o, 2, o.a.z);
		remainder = slice_step(slices, remainder, o, 2, o.b.z+1);
		slices.emplace_back(remainder.a, remainder.b);
		*/

		return slices;
	}

	std::vector<Cuboid> add (Cuboid oth) {
		std::vector<Cuboid> l = slice(oth);	
		std::vector<Cuboid> r = oth.slice(Cuboid(a, b));	
		std::vector<Cuboid> sum { l };
		for (Cuboid c : r) {
			if (std::find(std::begin(l), std::end(l), c) == std::end(l))
				sum.push_back(c);
		}
		return sum;
	}

	std::vector<Cuboid> sub (Cuboid oth) {
		std::vector<Cuboid> l = slice(oth);	
		std::vector<Cuboid> r = oth.slice(Cuboid(a, b));	
		std::vector<Cuboid> sum;
		for (Cuboid c : l) {
			if (std::find(std::begin(r), std::end(r), c) == std::end(r))
				sum.push_back(c);
		}
		return sum;
	}
};


class Rule {
	public:
	Rule (bool on, Cuboid c): on(on), c(c) { }
	Rule (std::string raw): c() { parse(raw); }

	std::pair<int, int> parse_range (std::string raw) {
		raw = raw.substr(2);
		auto dotdot = raw.find("..");
		return std::pair<int, int> { 
			std::stoi(raw.substr(0, dotdot)),
			std::stoi(raw.substr(dotdot+2))
		};	
	}

	void parse (std::string raw) {
		auto space = raw.find(" ");
		on = raw.substr(0, space) == "on";
		raw = raw.substr(space+1);
		auto comma = raw.find(",");
		auto range_x = parse_range(raw.substr(0, comma));
		raw = raw.substr(comma+1);
		comma = raw.find(",");
		auto range_y = parse_range(raw.substr(0, comma));
		raw = raw.substr(comma+1);
		auto range_z = parse_range(raw);

		c.a.x = range_x.first;
		c.b.x = range_x.second;
		c.a.y = range_y.first;
		c.b.y = range_y.second;
		c.a.z = range_z.first;
		c.b.z = range_z.second;
	}

	bool on;
	Cuboid c;
};

std::vector<Rule> parse_rules (std::string raw) {
	std::vector<Rule> rules;
	while (raw.size() > 0) {
		auto newline = raw.find("\n");
		std::string line = raw;
		if (newline != std::string::npos) line = raw.substr(0, newline);
		if (line.size() > 0) rules.emplace_back(line);
		raw = raw.substr(line.size()+1);
	}
	return rules;
}


class Reactor {
	public:
	Reactor () { }

	std::vector<Cuboid> cubes;

	long count_on () {
		long sum = 0;
		for (auto c : cubes) { sum += c.volume(); }
		return sum;
	}

	void apply_rule (Rule r, int max_radius) {
		if (std::abs(r.c.a.x) > max_radius) return;
		std::vector<Cuboid> next;
		bool intersected = false;
		for (Cuboid c : cubes) {
			if (c.intersect(r.c)) {
				intersected = true;
				for (Cuboid a : c.sub(r.c)) next.push_back(a);
			} else next.push_back(c);
		}
		if (r.on) next.push_back(r.c);
		cubes.clear();
		for (Cuboid c : next) cubes.push_back(c);
		if (DEBUG) {
			std::cout << "rule " << r.on << " " << r.c.show() << std::endl;
			std::cout << "  size " << cubes.size() 
				<< ", count on " << count_on() << std::endl;

			int intersections = 0;
			for (int i=0; i<cubes.size(); i++)
			for (int j=i; j<cubes.size(); j++) {
				if (i == j) continue;
				if (cubes[i].intersect(cubes[j])) {
					intersections++; break;
				}
			}
			std::cout << "  intersections " << intersections << std::endl; 
		}
	}

	void apply_rules (std::vector<Rule> rs, int max_radius) {
		for (Rule r : rs) {
			apply_rule(r, max_radius);
		}	
	}
};


// tests

bool test_volume_unit () {
	Cuboid a { Vec(1,1,1), Vec(1,1,1) };
	if (DEBUG) {
		std::cout << a.volume() << " == " << 1 << std::endl;
	}
	return a.volume() == 1;
}

bool test_volume () {
	Cuboid a { Vec(10,10,10), Vec(12,12,12) };
	if (DEBUG) {
		std::cout << a.volume() << " == " << 27 << std::endl;
	}
	return a.volume() == 27;
}

bool test_cut (Cuboid a, int axis, int h) {
	auto res = a.cut(axis, h);
	int vol = 0;
	for (Cuboid c : res) vol += c.volume();
	if (DEBUG) {
		std::cout << a.show() << " cut by plane axis:"
			<<axis<<" h:"<<h<<" =" << std::endl;
		for (Cuboid c : res) std::cout << c.show() << std::endl;
		std::cout << vol << " == " << a.volume() << std::endl;
	}
	return vol == a.volume();
}

bool test_cut_unit () {
	return test_cut (Cuboid(Vec(1,1,1), Vec(1,1,3)), 2, 2);
}

bool test_cut_border () {
	return test_cut (Cuboid(Vec(1,1,1), Vec(5,5,5)), 0, 5);
}

bool test_op (Cuboid a, Cuboid b, std::vector<Cuboid> res, int volume) {
	int vol = 0;
	for (Cuboid c : res) vol += c.volume();
	int intersects = 0;
	for (int i=0; i<res.size(); i++) 
		for (int j=0; j<res.size(); j++) {
			if (i == j) continue;
			if (res[i].intersect(res[j])) {
				if (DEBUG) { 
					std::cout << "intersect " << i << " " << j << std::endl;
					std::cout << res[i].show() << std::endl;
					std::cout << res[j].show() << std::endl;
				}
				intersects ++;
			}
		}

	if (DEBUG) {
		std::cout << a.show() << " op " << std::endl;
		std::cout << b.show() << " = " << std::endl;
		for (Cuboid c : res) std::cout << c.show() << std::endl;
		std::cout << vol << " == " << volume << 
			" && intersections " << intersects << std::endl;
	}
	return vol == volume && intersects == 0;
}

bool test_add (Cuboid a, Cuboid b, int v) { 
	return test_op(a, b, a.add(b), v);
}

bool test_sub (Cuboid a, Cuboid b, int v) { 
	return test_op(a, b, a.sub(b), v); 
}

bool test_add_simple () {
	return test_add(
			Cuboid(Vec(4,4,4), Vec(4,4,6)),
			Cuboid(Vec(4,4,4), Vec(4,4,5)), 3);
}

bool test_add_separate () {
	return test_add(
			Cuboid(Vec(4,4,4), Vec(4,4,4)),
			Cuboid(Vec(5,5,5), Vec(5,5,5)), 2);
}

bool test_add_small () {
	return test_add(
			Cuboid(Vec(4,4,4), Vec(5,5,5)),
			Cuboid(Vec(5,5,5), Vec(6,6,6)), 15);
}

bool test_add_example () {
	return test_add(
			Cuboid(Vec(10,10,10), Vec(12,12,12)),
			Cuboid(Vec(11,11,11), Vec(13,13,13)), 46);
}

bool test_sub_simple () {
	return test_sub(
			Cuboid(Vec(4,4,4), Vec(4,4,6)),
			Cuboid(Vec(4,4,4), Vec(4,4,5)), 1);
}

bool test_sub_example () {
	return test_sub(
			Cuboid(Vec(10,10,10), Vec(12,12,12)),
			Cuboid(Vec(11,11,11), Vec(13,13,13)), 19);
}

void tests () {
	int succ = 0, fail = 0;
	if ( test_volume_unit() ) succ++; else fail++;
	if ( test_volume() ) succ++; else fail++;
	if ( test_cut_unit() ) succ++; else fail++;
	if ( test_cut_border() ) succ++; else fail++;
	if ( test_add_simple() ) succ++; else fail++;
	if ( test_add_separate() ) succ++; else fail++;
	if ( test_add_small() ) succ++; else fail++;
	if ( test_add_example() ) succ++; else fail++;
	if ( test_sub_simple() ) succ++; else fail++;
	if ( test_sub_example() ) succ++; else fail++;
	std::cout << "test results: " << 
		succ << "/" << succ+fail << std::endl;
}


int main (int argc, char *argv[]) {
	if (argv[1][0] == '-' && argv[1][1] == 't') {
		tests();
		return 0;
	}
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	auto rules = parse_rules(raw);

	Reactor reactor;
	reactor.apply_rules(rules, 50);
	std::cout << "init on [50x50x50]: " << reactor.count_on() << std::endl;

	Reactor reactor_inf;
	reactor_inf.apply_rules(rules, 99999999);
	std::cout << "init on [INFxINFxINF]: " << reactor_inf.count_on() << std::endl;

	return 0;
}

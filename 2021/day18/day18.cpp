#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <cmath>

const bool debug = false;

class Snail {
	public:
	Snail() {}
	Snail(std::string raw) { parse(raw); }
	Snail(const Snail *s) : left(s->left), right(s->right) { 
		if (s->left == -1) left_child = new Snail(s->left_child);
		if (s->right == -1) right_child = new Snail(s->right_child);
	} 

	Snail *left_child, *right_child;
	int left = -1, right = -1;

	void parse(std::string raw) {
		int indent = 0;
		int comma = 0;
		for (int i=0; i<raw.size(); i++) {
			if (raw[i] == '[') indent++; 
			if (raw[i] == ']') indent--;
			if (raw[i] == ',' && indent == 1) comma = i;
		}
		auto l = raw.substr(1, comma);
		auto r = raw.substr(comma+1, raw.size()-comma-2);
		if (l[0] == '[') left_child = new Snail(l);
		else left = std::stoi(l);
		if (r[0] == '[') right_child = new Snail(r);
		else right = std::stoi(r);
	}

	std::string show () {
		std::string rep = "[";
		if (left != -1) rep += std::to_string(left); 
		else rep += left_child->show();
		rep += ",";
		if (right != -1) rep += std::to_string(right); 
		else rep += right_child->show();
		return rep + "]";
	}

	Snail& operator+=(Snail& rhs) {
		Snail *temp = new Snail(this);
		left_child = temp;
		left = -1;
		right_child = new Snail(&rhs);
		right = -1;
		reduce();
		return *this; 
	}
	 
	friend Snail operator+(Snail lhs, Snail& rhs) {
		lhs += rhs; 
		return lhs; 
	}

	void reduce () {
		while (true) {
			if (debug) std::cout << show() << std::endl;
			bool dirty = false;
			explode(0, &dirty);
			if (dirty) continue;
			split(&dirty);
			if (dirty) continue;
			break;
		}
	}

	void spill_up (int spill) {
		if (spill > 0) {
			if (left == -1) left_child->spill_up(spill);
			else left += spill;
		}
		if (spill < 0) {
			if (right == -1) right_child->spill_up(spill);
			else right += -spill;
		}
	}

	int explode (int indent, bool *dirty) {
		int spill = 0;
		if (indent >= 3) {
			if (left == -1 && !*dirty) {
				if (debug) std::cout << "boom " << show() << " -> ";
				left = 0;
				if (right == -1) right_child->left += left_child->right;
				else right += left_child->right;
				int spill = -left_child->left;
				*dirty = true;
				if (debug) std::cout << show() << " spilled " << spill << std::endl;
				return spill;
			}
			if (right == -1 && !*dirty) {
				if (debug) std::cout << "boom " << show() << " -> ";
				right = 0;
				if (left == -1) left_child->right += right_child->left;
				else left += right_child->left;
				int spill = right_child->right;
				*dirty = true;
				if (debug) std::cout << show() << " spilled " << spill << std::endl;
				return spill;
			}
		} else {
			if (left == -1) {
				spill = left_child->explode(indent+1, dirty);
				if (spill > 0) {
					if (debug) std::cout << "catch " << spill << " " << show() << " -> ";
					if (right != -1) right += spill;
					else right_child->spill_up(spill);
					spill = 0;
					if (debug) std::cout << show() << std::endl;
				}
				if (spill < 0) return spill;
			}
			if (right == -1) {
				spill = right_child->explode(indent+1, dirty);
				if (spill < 0) {
					if (debug) std::cout << "catch " << spill << " " << show() << " -> ";
					if (left != -1) left += -spill;
					else left_child->spill_up(spill);
					spill = 0;
					if (debug) std::cout << show() << std::endl;
				}
				if (spill > 0) return spill;
			}
		}
		return spill;
	}

	void split(bool *dirty) {
		if (*dirty) return;
		if (left == -1) { left_child->split(dirty); }
		else if (left >= 10) {
			if (debug) std::cout << "split " << show() << " -> ";
			left_child = new Snail();
			left_child->left = std::floor(left/2.0);
			left_child->right = std::ceil(left/2.0);
			left = -1;
			*dirty = true;
			if (debug) std::cout << show() << std::endl;
			return;
		}
		if (*dirty) return;
		if (right == -1) { right_child->split(dirty); }
		else if (right >= 10) {
			if (debug) std::cout << "split " << show() << " -> ";
			right_child = new Snail();
			right_child->left = std::floor(right/2.0);
			right_child->right = std::ceil(right/2.0);
			right = -1;
			*dirty = true;
			if (debug) std::cout << show() << std::endl;
			return;
		}
	}

	int magnitude () {
		int mag = 0;
		if (left == -1) mag += 3 * left_child->magnitude();
		else mag += 3 * left;
		if (right == -1) mag += 2 * right_child->magnitude();
		else mag += 2 * right;
		return mag;
	}
};

std::vector<Snail> parse (std::string raw) {
	std::vector<Snail> numbers;
	while (raw.size() > 0) {
		auto newline = raw.find("\n");
		if (newline != std::string::npos) {
			numbers.emplace_back(raw.substr(0, newline));
			raw = raw.substr(newline+1);
		} else {
			numbers.emplace_back(raw);
		}
	}
	return numbers;
}

int get_largest_perm (std::vector<Snail> nums) {
	int max = 0;
	for (int i=0; i<nums.size(); i++) {
		for (int j=0; j<nums.size(); j++) {
			if (j==i) continue;
			Snail *a = new Snail(nums[i]);
			Snail *b = new Snail(nums[j]);
			int mag = (*a + *b).magnitude();
			if (debug) std::cout << i << " " << j << " " << mag << std::endl;
			max = std::max(max, mag);
		}
	}
	return max;
}

int main (int argc, char *argv[]) {
	std::string raw;
	std::getline(std::ifstream(argv[1]), raw, '\0');
	std::vector<Snail> numbers = parse(raw);
	
	std::cout << "largest sum: " << get_largest_perm (numbers) << std::endl;

	int limit = 100000;
	if (argc > 2) limit = atoi(argv[2]);

	Snail sum { numbers[0] };
	for (int i=1; i<numbers.size() && i<limit; i++) {
		sum += numbers[i];
		if (debug)
		   	std::cout << "sum: " << sum.show() 
				<< ", mag: " << sum.magnitude() << std::endl;
	}
	std::cout << "sum: " << sum.show() 
		<< ", mag: " << sum.magnitude() << std::endl;

	return 0;
}

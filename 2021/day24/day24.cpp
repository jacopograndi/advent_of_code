#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <set>

#include "gtest/gtest.h"

class Alu {
	private:
	long v[4];

	public:
	Alu() {
		v[0] = 0; v[1] = 0; v[2] = 0; v[3] = 0;   
	}

	std::string show () {
		return "(" +
			std::to_string(v[0]) + ", " +
			std::to_string(v[1]) + ", " +
			std::to_string(v[2]) + ", " +
			std::to_string(v[3]) +
			")";
	}

	int operator[](const int& i) const { return v[i]; }

	void inp (int i, int val) { v[i] = val; }
	void add (int i, int val) { v[i] = v[i]+val; }
	void mul (int i, int val) { v[i] = v[i]*val; }
	void div (int i, int val) { v[i] = v[i]/val; }
	void mod (int i, int val) { v[i] = v[i]%val; }
	void eql (int i, int val) { v[i] = v[i]==val ? 1 : 0; }
	void op (int o, int i, int j, bool imm) {
		int val = imm ? j : v[j];
		if (o==0) inp(i, val);
		else if (o==1) add(i, val);
		else if (o==2) mul(i, val);
		else if (o==3) div(i, val);
		else if (o==4) mod(i, val);
		else if (o==5) eql(i, val);
	}
};

TEST (alu, exisits) {
	Alu alu;
}

TEST (alu, variables) {
	Alu alu;
	ASSERT_EQ(0, alu[0]);
	ASSERT_EQ(0, alu[1]);
	ASSERT_EQ(0, alu[2]);
	ASSERT_EQ(0, alu[3]);
}

TEST (alu, inp) {
	Alu alu;
	alu.inp(2, 5);
	ASSERT_EQ(5, alu[2]);
}

TEST (alu, add) {
	Alu alu;
	alu.inp(2, 5); alu.inp(1, 6);
	alu.add(1, alu[2]);
	ASSERT_EQ(11, alu[1]);
}

TEST (alu, mul) {
	Alu alu;
	alu.inp(0, 5); alu.inp(3, 6);
	alu.mul(3, alu[0]);
	ASSERT_EQ(30, alu[3]);
}

TEST (alu, div) {
	Alu alu;
	alu.inp(0, 31); alu.inp(1, 6);
	alu.div(0, alu[1]);
	ASSERT_EQ(5, alu[0]);
	alu.inp(0, -31); alu.inp(1, 6);
	alu.div(0, alu[1]);
	ASSERT_EQ(-5, alu[0]);
}

TEST (alu, mod) {
	Alu alu;
	alu.inp(0, 32); alu.inp(1, 6);
	alu.mod(0, alu[1]);
	ASSERT_EQ(2, alu[0]);
}

TEST (alu, eql) {
	Alu alu;
	alu.inp(0, 32); alu.inp(1, 43);
	alu.eql(0, alu[1]);
	ASSERT_EQ(0, alu[0]);
	alu.inp(0, 5); alu.inp(1, 5);
	alu.eql(0, alu[1]);
	ASSERT_EQ(1, alu[0]);
}

TEST (alu, op) {
	Alu alu;
	alu.op(0, 0, 10, true); alu.op(0, 1, 7, true);
	alu.op(5, 0, 1, false);
	ASSERT_EQ(0, alu[0]);
}


class Rule {
	public:
	Rule () { }
	Rule (std::string raw) { parse(raw); }
	int op, i, j;
	bool imm;

	void parse (std::string raw) {
		auto rop = raw.substr(0, 3);
		if (rop == "inp") op = 0;
		else if (rop == "add") op = 1;
		else if (rop == "mul") op = 2;
		else if (rop == "div") op = 3;
		else if (rop == "mod") op = 4;
		else if (rop == "eql") op = 5;

		auto par = raw.substr(4);
		i = par[0] -'w';
		if (op > 0) {
			if (par[2] == 'w' || par[2] == 'x' ||
					par[2] == 'y' || par[2] == 'z') {
				imm = false;
				j = par[2] -'w';
			}
			else  {
				imm = true;
				j = std::stoi( par.substr(2) );
			}
		}
	}
};

class State {
	public:
	State (): alu(), pc(0) { }
	State (std::vector<int> in): alu(), input(in), pc(0) { }
	State (std::string raw): alu(), pc(0) { parse(raw); }

	Alu alu;
	std::vector<int> input;
	int pc;

	void parse (std::string raw) {
		for (char c : raw) {
			input.push_back(std::stoi(std::string { c }));
		}
	}

	void rule (const Rule &rule) {
		if (rule.op == 0) {
			alu.op(rule.op, rule.i, input[pc], true);
			pc++;
		} else
			alu.op(rule.op, rule.i, rule.j, rule.imm);
	}
};

TEST (rule, translate_one) {
	std::string raw { "inp x" };
	Rule r { raw };
	ASSERT_EQ(0, r.op);
	ASSERT_EQ(1, r.i);
}

TEST (rule, translate_two) {
	std::string raw { "mod w x" };
	Rule r { raw };
	ASSERT_EQ(4, r.op);
	ASSERT_EQ(0, r.i);
	ASSERT_EQ(1, r.j);
}

TEST (rule, translate_immediate) {
	std::string raw { "add z 20" };
	Rule r { raw };
	ASSERT_EQ(1, r.op);
	ASSERT_EQ(3, r.i);
	ASSERT_EQ(20, r.j);
}


class Program {
	public:
	Program(std::string raw, State s): state(s) { parse(raw); }
	Program(std::vector<Rule> r, State s): rules(r), state(s) { }

	State state;
	std::vector<Rule> rules;

	void parse (std::string raw) {
		while (raw.size() > 0) {
			auto newline = raw.find("\n");
			if (newline == std::string::npos) {
				rules.push_back(Rule { raw });
				break; 
			} else {
				rules.push_back(Rule { raw.substr(0, newline) });
				raw = raw.substr(newline+1);
			}
		}
	}

	void run () {
		for (Rule &rule : rules) {
			state.rule(rule);
		}
	}
};

TEST (program, exists) {
	State p;
}

TEST (program, from_input) {
	std::vector<int> input { 1, 2, 2, 4 };
	State p { input };
}

TEST (program, apply_rule) {
	std::vector<int> input { 1 };
	State p { input };
	p.rule(Rule { "inp x" });
	ASSERT_EQ(1, p.alu[1]);
}

TEST (program, apply_negate) {
	std::vector<int> input { 6 };
	State p { input };
	p.rule(Rule { "inp x" });
	p.rule(Rule { "mul x -1" });
	ASSERT_EQ(-6, p.alu[1]);
}

TEST (program, apply_triple) {
	Program p {
		{ 
			Rule { "inp z" },
			Rule { "inp x" },
			Rule { "mul z 3" },
			Rule { "eql z x" }
		},
		State { std::string { "26" } } 
	};
	p.run();
	ASSERT_EQ(1, p.state.alu[3]);
}

TEST (program, bits) {
	std::string raw;
	std::getline(std::ifstream("test0.txt"), raw, '\0');
	Program p { raw, State { std::vector<int>{ 15 }} };
	p.run();
	ASSERT_EQ(1, p.state.alu[0]);
	ASSERT_EQ(1, p.state.alu[1]);
	ASSERT_EQ(1, p.state.alu[2]);
	ASSERT_EQ(1, p.state.alu[3]);
}

class Set {
	public:
	Set () { }
	Set (const Set &s): v(s.v)  { }
	Set (int min, int max) {
		for (int i=min; i<max+1; i++) v.push_back(i);
	}

	std::vector<int> v;

	std::string show () {
		std::string rep = "(Set, [";
		for (int i=0; i<v.size(); i++) {
			rep += std::to_string(v[i]);
			if (i < v.size()-1) rep += ", ";
		}
		rep += "])";
		return rep;
	}
	
	Set add (int n) const {
		Set s;
		for (int i=0; i<v.size(); i++)
			s.v.push_back(v[i] + n);
		return s;
	};

	Set sub (int n) const { return add(-n); };

	Set intersect (const Set &oth) const { 
		Set s;
		for (int i=0; i<v.size(); i++) {
			if (std::find(
					std::begin(oth.v), std::end(oth.v), v[i]
				) != std::end(oth.v))
		   	{
				s.v.push_back(v[i]);
			}
		}
		return s;
	}

	Set diff (const Set &oth) const { 
		Set s;
		for (int i=0; i<v.size(); i++) {
			if (std::find(
					std::begin(oth.v), std::end(oth.v), v[i]
				) == std::end(oth.v))
		   	{
				s.v.push_back(v[i]);
			}
		}
		return s;
	}
};

TEST (set, exists) {
	Set set;
}

TEST (set, range) {
	Set set(1, 9);
	ASSERT_EQ(1, set.v[0]);
	ASSERT_EQ(7, set.v[6]);
	ASSERT_EQ(9, set.v[8]);
}

TEST (set, add) {
	Set base(1, 9);
	Set set { base.add(3) };
	ASSERT_EQ(4, set.v[0]);
	ASSERT_EQ(10, set.v[6]);
	ASSERT_EQ(12, set.v[8]);
}

TEST (set, sub) {
	Set base(1, 9);
	Set set { base.sub(3) };
	ASSERT_EQ(-2, set.v[0]);
	ASSERT_EQ(4, set.v[6]);
	ASSERT_EQ(6, set.v[8]);
}

TEST (set, intersect) {
	Set a(1, 9);
	Set b(4, 12);
	Set set { a.intersect(b) };
	ASSERT_EQ(6, set.v.size());
}

TEST (set, diff) {
	Set a(1, 9);
	Set b(4, 12);
	Set set { a.diff(b) };
	ASSERT_EQ(3, set.v.size());
}


int fairy[14] { 1,1,1,1,26,26,1,1,26,26,1,26,26,26 };
int magic[14] { 13,12,12,10,-11,-13,15,10,-2,-6,14,0,-15,-4 };
int witch[14] { 8,13,8,10,12,1,13,5,10,3,2,2,12,7 };

std::vector<Set> good_ids (
		std::vector<Set> ins, std::vector<Set> z_prev, int n) 
{
	if (z_prev.size() > 5) { return {}; }
	if (n == 14) { 
		if (z_prev.size() <= 0) return ins;
		else return {}; 
	}
	std::vector<Set> z { z_prev };

	Set in(1, 9);
	Set x;
	if (z.size() > 0) {
		x = z[z.size()-1].add(magic[n]);
		if (fairy[n] == 26) z.erase(z.end()-1);
	}
	Set other = in.diff(x);
	Set nop = in.diff(other);

	for (int i=0; i<n; i++) std::cout << " ";
	std::cout << n << " " << z.size();
	std::cout << ", " << nop.show() << std::endl;
	
	std::vector<Set> ins_nop = ins;
	if (other.v.size() > 0) {
		std::vector<Set> ins_oth = ins; ins_oth.push_back(other);
		std::vector<Set> z_n = z; z_n.push_back(other.add(witch[n]));
		ins = good_ids(ins_oth, z_n, n+1);
	}
	if (nop.v.size() > 0) { 
		ins_nop.push_back(nop);
		std::vector<Set> ins_f = good_ids(ins_nop, z, n+1);
		if (ins.size() == 0) {
			ins = ins_f;
		}
	}
	return ins;
}

TEST (ranges, view) {
	auto ids = good_ids({}, {}, 0);
	for (auto s : ids) {
		std::cout << s.show() << std::endl;
	}
	ASSERT_NE(0, ids.size());
}

long min_temp = 999999999999;

std::vector<int> look_rec (
		std::vector<Set> &s, Program &p, std::vector<int> res, int n, bool big) 
{
	if (n == 14) {
		Program test { p.rules, State { res } };
		test.run();
		//if (std::abs(test.state.alu[3]) <= 15) {
		//if (std::abs(test.state.alu[3]) <= 455) {
		if (std::abs(test.state.alu[3]) <= min_temp) {
			min_temp = std::abs(test.state.alu[3]);
			for (int i=0; i<n; i++) std::cout << res[i];
				std::cout << " " << test.state.alu[3];
			std::cout << std::endl;
		}
		if (test.state.alu[3] == 0) return res;
		else return {};
	}
	for (int i=s[n].v.size()-1; i>=0; i--) {
		std::vector<int> r { res };
		int num = s[n].v[i];
		if (!big) num = s[n].v[s[n].v.size()-1-i];
		r.push_back(num);
		auto f = look_rec(s, p, r, n+1, big);
		if (f.size() > 0) return f;
	}
	return {};
}


std::vector<int> look (Program p, bool big) {
	auto ids = good_ids({}, {}, 0);
	if (big) {
		ids[0].v.erase(ids[0].v.end()-1);
		ids[0].v.erase(ids[0].v.end()-1);
		ids[0].v.erase(ids[0].v.end()-1);
		ids[0].v.erase(ids[0].v.end()-1);
	} else {
		ids[0].v = { 1 };
		//ids[1].v = { 1 };
		//ids[2].v = { 1 };
		ids[3].v = { 2 };
		//ids[4].v = { 1 };

		ids[10].v = { 1 };
		ids[11].v = { 3 };
		ids[12].v = { 1 };
		ids[13].v = { 5 };
	}
	return look_rec(ids, p, {}, 0, big);
}


int main (int argc, char *argv[]) {
	if (argc > 1 && std::string { argv[1] } == "-t") {
		testing::InitGoogleTest(&argc, argv);
		return RUN_ALL_TESTS();
	}
	std::string raw;
	std::getline(std::ifstream("day24_input.txt"), raw, '\0');
	
	auto id = 0;
	Program p { raw, State { } };
	if (argc > 1 && std::string { argv[1] } == "big") {
		look(p, true);
	}
	if (argc > 1 && std::string { argv[1] } == "small") {
		look(p, false);
	}
	return 0;
}

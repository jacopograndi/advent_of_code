#include <iostream>
#include <fstream>
#include <string>

#include "gtest/gtest.h"


class Board {
	private:
	std::vector<int> tiles;

	int at_uncheck (int i, int j) { return i+j*sx; }
	int at (int i, int j) const { return (i%sx)+(j%sy)*sx; }

	public:
	int sx, sy;

	Board(const Board &b): sx(b.sx), sy(b.sy), tiles(b.tiles) { }
	Board(int sx, int sy): sx(sx), sy(sy) {
		for (int j=0; j<sy; j++)
			for (int i=0; i<sx; i++)
				tiles.push_back(0);
	}
	Board(std::string raw) {
		sy = 0;
		while (raw.size() > 0) {
			std::string line = raw;
			auto token = raw.find("\n");
			if (token != std::string::npos) {
				line = raw.substr(0, token);
				raw = raw.substr(token+1);
			} else raw = "";
			for (int i=0; i<line.size(); i++) {
				if (line[i] == '.') tiles.push_back(0);
				if (line[i] == '>') tiles.push_back(1);
				if (line[i] == 'v') tiles.push_back(2);
			}
			sx = line.size();
			sy ++;
		}
	}

	int get (int i, int j) const { return tiles[at(i, j)]; }
	void set (int i, int j, int v) { tiles[at(i, j)] = v; }

	friend std::ostream& operator<< (
			std::ostream& stream, const Board& board) 
	{
		stream << "Board (" 
			<< board.sx << ", " << board.sy << ")" << std::endl;
		for (int j=0; j<board.sy; j++) {
			for (int i=0; i<board.sx; i++) {
				int tile = board.get(i, j);
				if (tile == 0) stream << ".";
				if (tile == 1) stream << ">";
				if (tile == 2) stream << "v";
			}
			stream << std::endl;
		}
		return stream;
	}
};

TEST (board, exist) {
	Board b { 4, 4 };
}

TEST (board, modify) {
	Board b { 4, 4 };
	b.set(3, 3, 1);
	ASSERT_EQ(1, b.get(3, 3));
}

TEST (board, donut) {
	Board b { 4, 4 };
	b.set(4, 4, 1);
	ASSERT_EQ(1, b.get(0, 0));
}

TEST (board, parse_ez) {
	Board b { ".>v" };
	ASSERT_EQ(0, b.get(0, 0));
	ASSERT_EQ(1, b.get(1, 0));
	ASSERT_EQ(2, b.get(2, 0));
}

TEST (board, parse_test_txt) {
	std::string raw;
	std::getline(std::ifstream("test0.txt"), raw, '\0');
	Board b { raw };
	ASSERT_EQ(0, b.get(0, 0));
	ASSERT_EQ(1, b.get(3, 0));
	ASSERT_EQ(2, b.get(0, 3));
	ASSERT_EQ(2, b.get(4, 6));
}


class Engine {
	private:
	int t;

	public:
	Engine (Board b): board(b), t(0) { }

	Board board;

	friend std::ostream& operator<< (
			std::ostream& stream, const Engine& e) 
	{
		stream << "Engine (step: " << e.t << ")" << std::endl;
		stream << e.board;
		return stream;
	}

	int step_herd(int h, int move) {
		int changes = 0;
		Board next { board }; 
		for (int j=0; j<board.sy; j++) {
			for (int i=0; i<board.sx; i++) {
				if (board.get(i, j) == h) {
					if (move == 0) {
						if (board.get(i+1, j) == 0) {
							next.set(i+1, j, board.get(i, j));
							next.set(i, j, 0);
							changes ++;
						}
					} 
					if (move == 1) {
						if (board.get(i, j+1) == 0) {
							next.set(i, j+1, board.get(i, j));
							next.set(i, j, 0);
							changes ++;
						}
					}
				}
			}
		}
		board = next;
		return changes;
	}

	int step () {
		t++;
		int changes = 0;
		changes += step_herd(1, 0);
		changes += step_herd(2, 1);
		return changes;
	}

	void run () {
		int changes = step();
		if (changes > 0) run();
	}

	int get_t() { return t; }
};

TEST (engine, step_ez) {
	Engine engine { Board { "...>>>>>..." } };
	engine.step();
	ASSERT_EQ(1, engine.board.get(8, 0));
	engine.step();
	ASSERT_EQ(0, engine.board.get(8, 0));
}

TEST (engine, step_blocking) {
	std::string raw;
	std::getline(std::ifstream("test1.txt"), raw, '\0');
	Engine engine { Board { raw } };
	engine.step();
	ASSERT_EQ(2, engine.board.get(7, 2));
}

TEST (engine, step_run) {
	std::string raw;
	std::getline(std::ifstream("test2.txt"), raw, '\0');
	Engine engine { Board { raw } };
	engine.run();
}


int main (int argc, char *argv[]) {
	if (argc > 1 && std::string { argv[1] } == "-t") {
		testing::InitGoogleTest(&argc, argv);
		return RUN_ALL_TESTS();
	}

	std::string raw;
	std::getline(std::ifstream("day25_input.txt"), raw, '\0');
	Engine engine { Board { raw } };
	engine.run();
	std::cout << "steps: " << engine.get_t() << std::endl;

	return 0;
}

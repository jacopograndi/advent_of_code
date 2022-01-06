#ifndef UTILS_H
#define UTILS_H

void split (std::vector<std::string> &vec, std::string str, std::string del) {
    auto token = str.find(del);
    if (token != std::string::npos) {
        vec.push_back(str.substr(0, token));
        split(vec, str.substr(token+del.size()), del);
    } else { vec.push_back(str); }
}

#endif

#ifndef UTILS_H
#define UTILS_H

void split (std::vector<std::string> &vec, std::string str, std::string del) {
    auto token = str.find(del);
    if (token != std::string::npos) {
        vec.push_back(str.substr(0, token));
        split(vec, str.substr(token+del.size()), del);
    } else { vec.push_back(str); }
}

// trim
static inline void ltrim(std::string &s) {
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
        return !std::isspace(ch);
    }));
}
static inline void rtrim(std::string &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
        return !std::isspace(ch);
    }).base(), s.end());
}
static inline void trim(std::string &s) { ltrim(s); rtrim(s); }


#endif

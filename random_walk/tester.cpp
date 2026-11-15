
#include <cstdio>
#include <cstring>
#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>
using namespace std;
template <typename T>
T extract(string str)
{
	stringstream ss;

	/* Storing the whole string into string stream */
	ss << str;

	/* Running loop till the end of the stream */
	string temp;
	T found;
	while (!ss.eof()) {

		/* extracting word by word from stream */
		ss >> temp;

		/* Checking the given word is integer or not */
		if (stringstream(temp) >> found)
			return found;

		/* To save from space at the end of string */
		temp = "";
	}
	return found;
}

bool isValid(string s, bool is3D)
{
	if (!is3D)
		return regex_match(s, regex("(The a|A)verage (disposition|displacement) of a one dimensional random walk in \\d+ (steps|steps is): \\d+.\\d units?"));
	else
		return regex_match(s, regex("(The a|A)verage (disposition|displacement) of a three dimensional random walk in \\d+ (steps is|steps): \\d+.\\d units?"));
}

int main(int argc, char const* argv[])
{

	ifstream test_in(argv[1]); /* This stream reads from test's input file   */
	ifstream test_out(argv[2]); /* This stream reads from test's output file  */
	ifstream user_out(argv[3]); /* This stream reads from user's output file  */

	/* Your code here */
	/* If user's output is correct, return 0, otherwise return 1       */

	char inp[90]; //for test output only
	string userOutString;
	int usteps, asteps;
	double uans, ans;
	double acc = 5.0;

	//main problem
	getline(user_out, userOutString);
	if (!isValid(userOutString, false))
		return 1;
	usteps = extract<int>(userOutString);
	uans = extract<double>(userOutString.substr(userOutString.size()-11,userOutString.size()));

	test_out.getline(inp, 90);
	sscanf(inp, "Average disposition of a one dimensional random walk in %d steps is: %lf units", &asteps, &ans);

	int out = ((ans + acc >= uans && uans >= ans - acc) ? 0 : 1);

	// bonus
	inp[0] = '\0';
	test_out.getline(inp, 90);
	if (strlen(inp) > 20) {
		sscanf(inp, "Average disposition of a three dimensional random walk in %d steps is: %lf units", &asteps, &ans);

		getline(user_out, userOutString);
		if (!isValid(userOutString, true))
			return 1;
		usteps = extract<int>(userOutString);
		uans = extract<double>(userOutString.substr(userOutString.size()-11,userOutString.size()));

		return out + ((ans + acc >= uans && uans >= ans - acc) ? 0 : 1);
	}
	return out;
}

//int main()
//{
//	string s = "Average disposition of a one dimensional random walk in 9385 steps is: 62.6 unit";
//	cout << isValid(s,false) << endl;
//	int n = extract<int>(s);
//	double d = extract<double>(s.substr(s.size()-11,s.size()));
//	cout << endl << n << endl;
//	cout << endl << d << endl;
//	
//
//}

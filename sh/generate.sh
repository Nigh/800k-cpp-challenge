#!/bin/sh
cat <<EOF
#include <iostream>
using namespace std;

int main()
{
    cout << "请给出一个不多于5位的正整数:";
    int x;
    cin >> x;

    switch (x) {
EOF

awk 'BEGIN {
	units="个十百千万"
	for(x=1;x<=99999;++x) {
		xlen=length(x);
		rx="";
		printf("    case %d:\n",x);
		printf("        cout << \"是%d位数\" << endl;\n",xlen);
		for (i=0;i<xlen;++i) {
			k=substr(x,xlen-i,1);
			printf("        cout << \"%s位数是：%d\" << endl;\n",substr(units,i+1,1),k);
			rx=sprintf("%s%d",rx,k);
		}
		printf("        cout << \"倒过来是：%s\" << endl;\n",rx);
		printf("        break;\n");
	}
}'

echo
echo "    }"
echo "}"

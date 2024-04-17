#include "GaloisField.h"

int main() {
    try {
        GaloisField a(5);
        GaloisField b(1234576);
        GaloisField c = a + b;
        std::cout << "a: " << a << std::endl;
        std::cout << "b: " << b << std::endl;
        std::cout << "a + b: " << c << std::endl;
        std::cout << "a == b: " << (a == b) << std::endl;
        std::cout << "a != b: " << (a != b) << std::endl;
        a += b;
        std::cout << "a += b, a: " << a << std::endl;
        std::cout << std::endl;

        GaloisField d(10);
        GaloisField e(20);
        GaloisField f = d - e;
        std::cout << "d: " << d << std::endl;
        std::cout << "e: " << e << std::endl;
        std::cout << "d - e: " << f << std::endl;
        std::cout << "d >= e: " << (d >= e) << std::endl;
        std::cout << "d <= e: " << (d < e) << std::endl;
        d -= e;
        std::cout << "d -= e, d: " << d << std::endl;
        std::cout << std::endl;

        GaloisField g(308646);
        GaloisField h(4);
        GaloisField i = g * h;
        std::cout << "g: " << g << std::endl;
        std::cout << "h: " << h << std::endl;
        std::cout << "g * h: " << i << std::endl;
        std::cout << "g > h: " << (g > h) << std::endl;
        std::cout << "g < h: " << (g < h) << std::endl;
        g *= h;
        std::cout << "g *= h, g: " << g << std::endl;
        std::cout << std::endl;

        GaloisField j(5); 
        GaloisField k(1234576);
        GaloisField l = j / k;
        std::cout << "j: " << j << std::endl;
        std::cout << "k: " << k << std::endl;
        std::cout << "j / k: " << l << std::endl;
        j /= k;
        std::cout << "j /= k, j: " << j << std::endl;
        std::cout << std::endl;

        std::cout << "Enter a number: ";
        GaloisField wpis;
        std::cin >> wpis;
        std::cout << "You entered: " << wpis << std::endl;

    } catch (const std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }
    return 0;
}

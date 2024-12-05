#include <iostream>
#include <cmath>

class Vector3 {
public:
    float x, y, z;

    Vector3(float x, float y, float z) : x(x), y(y), z(z) {}

    std::string str() const {
        return "(" + std::to_string(x) + ", " + std::to_string(y) + ", " + std::to_string(z) + ")";
    }

    float magnitude() const {
        return std::sqrt(x * x + y * y + z * z);
    }

    float dot(const Vector3& other) const {
        return x * other.x + y * other.y + z * other.z;
    }

    Vector3 normalize() const {
        float mag = magnitude();
        if (mag > 0) {
            return *this / mag;
        } else {
            return ZERO;
        }
    }

    bool operator==(const Vector3& other) const {
        return x == other.x && y == other.y && z == other.z;
    }

    Vector3 operator+(const Vector3& other) const {
        return Vector3(x + other.x, y + other.y, z + other.z);
    }

    Vector3 operator-(const Vector3& other) const {
        return Vector3(x - other.x, y - other.y, z - other.z);
    }

    Vector3 operator*(float scalar) const {
        return Vector3(x * scalar, y * scalar, z * scalar);
    }

    Vector3 operator/(float scalar) const {
        return Vector3(x / scalar, y / scalar, z / scalar);
    }

    friend std::ostream& operator<<(std::ostream& os, const Vector3& vec) {
        os << vec.str();
        return os;
    }

    static const Vector3 ZERO;
};

const Vector3 Vector3::ZERO(0, 0, 0);

// int main() {
//     Vector3 v1(1.0f, 2.0f, 3.0f);
//     Vector3 v2(4.0f, 5.0f, 6.0f);

//     std::cout << "v1: " << v1 << std::endl;
//     std::cout << "v2: " << v2 << std::endl;
//     std::cout << "v1 + v2: " << v1 + v2 << std::endl;
//     std::cout << "v1 - v2: " << v1 - v2 << std::endl;
//     std::cout << "v1 * 2: " << v1 * 2.0f << std::endl;
//     std::cout << "v1 / 2: " << v1 / 2.0f << std::endl;
//     std::cout << "Dot product: " << v1.dot(v2) << std::endl;
//     std::cout << "Magnitude of v1: " << v1.magnitude() << std::endl;
//     std::cout << "Normalized v1: " << v1.normalize() << std::endl;

//     return 0;
// }
#include <gtest/gtest.h>
#include "../src/vector.cpp"

TEST(Vector3Test, Print) {
    Vector3 v(0, 0, 0);
    std::cout << v << std::endl;
}

TEST(Vector3Test, Magnitude) {
    EXPECT_FLOAT_EQ(Vector3(0, 0, 0).magnitude(), 0);
    EXPECT_FLOAT_EQ(Vector3(-2, 2, 1).magnitude(), 3);
}

TEST(Vector3Test, Add) {
    EXPECT_EQ(Vector3(0, 0, 0) + Vector3(0, 0, 0), Vector3(0, 0, 0));
    EXPECT_EQ(Vector3(1, 0, 0) + Vector3(0, 0, 0), Vector3(1, 0, 0));
    EXPECT_EQ(Vector3(1, 1, 1) + Vector3(1, 1, 1), Vector3(2, 2, 2));
    EXPECT_EQ(Vector3(1, 1, 1) + Vector3(-1, -1, -1), Vector3(0, 0, 0));
}

TEST(Vector3Test, Multiply) {
    EXPECT_EQ(Vector3(0, 0, 0) * 3, Vector3(0, 0, 0));
    EXPECT_EQ(Vector3(1, 0, 0) * 3, Vector3(3, 0, 0));
    EXPECT_EQ(Vector3(1, 1, 1) * 3, Vector3(3, 3, 3));
    EXPECT_EQ(Vector3(1, 1, 1) * 0, Vector3(0, 0, 0));
}

TEST(Vector3Test, Divide) {
    EXPECT_EQ(Vector3(0, 0, 0) / 3, Vector3(0, 0, 0));
    EXPECT_EQ(Vector3(3, 0, 0) / 3, Vector3(1, 0, 0));
    EXPECT_EQ(Vector3(9, 9, 9) / 3, Vector3(3, 3, 3));
}

TEST(Vector3Test, Normalize) {
    EXPECT_EQ(Vector3(0, 0, 0).normalize(), Vector3::ZERO.normalize());
    EXPECT_EQ(Vector3(1, 1, 1).normalize(), Vector3(2, 2, 2).normalize());
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
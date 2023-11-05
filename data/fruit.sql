/*
 Navicat Premium Data Transfer

 Source Server         : test-sqlite
 Source Server Type    : SQLite
 Source Server Version : 3035005
 Source Schema         : main

 Target Server Type    : SQLite
 Target Server Version : 3035005
 File Encoding         : 65001

 Date: 05/11/2023 12:17:47
*/

PRAGMA foreign_keys = false;

-- ----------------------------
-- Table structure for fruit
-- ----------------------------
DROP TABLE IF EXISTS "fruit";
CREATE TABLE "fruit" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT NOT NULL
);

-- ----------------------------
-- Auto increment value for fruit
-- ----------------------------

PRAGMA foreign_keys = true;

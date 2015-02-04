var searchIndex = {};
searchIndex['graph'] = {"items":[[0,"","graph",""],[1,"GraphPath","","A struct used to represent a path in a graph."],[0,"graphs","","This module contains default implementations for graphs and edges"],[1,"WeightedEdge","graph::graphs","A default implementation of a weighted edge that can be used in graph implementations."],[1,"UnweightedEdge","","A default implementation of a weighted edge that can be used in graph implementations."],[1,"UndirectedAdjacencyListGraph","","An undirected graph represented by an adjacency list."],[10,"hash","","",0],[10,"clone","","",0],[10,"new","","",0],[10,"get_weight","","",0],[10,"get_source","","",0],[10,"get_target","","",0],[10,"is_directed","","",0],[10,"eq","","",0],[10,"hash","","",1],[10,"clone","","",1],[10,"new","","",1],[10,"get_weight","","",1],[10,"get_source","","",1],[10,"get_target","","",1],[10,"is_directed","","",1],[10,"eq","","",1],[10,"clone","","",2],[10,"new","","",2],[10,"add_node","","",2],[10,"add_edge","","",2],[10,"get_nodes","","",2],[10,"get_node_neighbours","","",2],[10,"get_edge","","",2],[10,"get_edges","","",2],[10,"is_adjacent","","",2],[10,"is_node_in_graph","","",2],[10,"degree","","",2],[6,"Graph","graph","The `Graph` trait is used to implement common operations on a graph and provide implementations of graph algorithms\nthat use these operations so that concrete types of graphs can be implemented and the algorithms used on them."],[9,"new","","Creates a new instance of the graph.",3],[9,"add_node","","The method to add a node to the graph.",3],[9,"add_edge","","The method to add an edge to the graph between two nodes, specifying a weight.",3],[9,"get_nodes","","The method to return a vector of IDs of all nodes in the graph.",3],[9,"get_node_neighbours","","The method to get the list of nodes that are adjacent to a node.",3],[9,"get_edge","","The method to get edge between two nodes.",3],[9,"get_edges","","The method to get the edge set in the graph.",3],[9,"is_adjacent","","The method to check if two nodes are adjacent.",3],[9,"is_node_in_graph","","The method to check if a node is in the graph.",3],[9,"degree","","The method to return the degree of a node.",3],[10,"dijkstras_shortest_path","","Performs Dijkstra's shortest path algorithm on the graph.",3],[10,"dijkstras_shortest_paths","","Performs Dijkstra's shortest path algorithm on the graph.",3],[10,"diameter_path","","Finds the diameter of the graph.",3],[10,"k_core_decomposition","","Finds the k core of each vertex in the graph.",3],[10,"kruskal_min_spanning_tree","","Creates a minimum spanning tree of the graph using Kruskal's algorithm.",3],[6,"Edge","",""],[9,"new","","",4],[9,"get_weight","","",4],[9,"get_source","","",4],[9,"get_target","","",4],[9,"is_directed","","",4],[10,"clone","","",5],[10,"fmt","","",5],[10,"get_distance","","Retrieves the distance of the `GraphPath`",5],[10,"get_path","","Retrieves the path.",5]],"paths":[[1,"WeightedEdge"],[1,"UnweightedEdge"],[1,"UndirectedAdjacencyListGraph"],[6,"Graph"],[6,"Edge"],[1,"GraphPath"]]};
searchIndex['disjoint_set'] = {"items":[[0,"","disjoint_set",""],[1,"DisjointSet","","Struct that represents the [Disjoint-Set](http://en.wikipedia.org/wiki/Disjoint-set_data_structure) data structure."],[10,"clone","","",0],[10,"new","","",0],[10,"make_set","","Makes a singleton set of the value inside the `DisjointSet`.",0],[10,"find","","Finds the value of the root of the set that the value belongs to and performs path compression on the visited nodes.",0],[10,"union","","Unions the two sets that each value belongs to using union by rank.",0]],"paths":[[1,"DisjointSet"]]};

initSearch(searchIndex);
